//! Entities, collections of components
//! 
//! An entity is a collection of components.
//! 
//! # Example
//! 
//! ```rust
//! use mecs::{Entity, DynStorage};
//! 
//! let entity: Entity<DynStorage> = mecs::entity![
//! 	DynStorage::new(23i32),
//! 	DynStorage::new("Test"),
//! 	DynStorage::new(5.257),
//! ];
//! 
//! let num: i32 = *entity.get().unwrap();
//! 
//! assert_eq!(num, 23);
//! ```

// Modules
#[cfg(test)]
mod test;

// Collections
use std::collections::HashMap;

// Traits
use std::fmt::Debug;
use std::iter::Iterator;

// Crate
use crate::{KeyType, Storage, Component};

// Macros
//--------------------------------------------------------------------------------------------------
	/// Creates an entity from it's components
	#[macro_export]
	macro_rules! entity
	{
		[ $( $cmpt:expr ),* $(,)? ] => {{
			
			let mut entity = $crate::Entity::new();
			
			$(
				entity.add( $cmpt );
			)*
			
			entity
		}};
	}
//--------------------------------------------------------------------------------------------------

// Types
//--------------------------------------------------------------------------------------------------
	/// An entity
	#[derive(PartialEq, Eq, Clone, Debug)]
	pub struct Entity<'a, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType,
	{
		/// All of the components
		components: HashMap<S::Id, S>,
	}
//--------------------------------------------------------------------------------------------------

// Impl
//--------------------------------------------------------------------------------------------------
	impl<'a, S, I> Entity<'a, S>
	where
		S: Storage<'a, Id = I>,
		I: KeyType + 'a,
	{
		// Constructors
		//--------------------------------------------------------------------------------------------------
			/// Creates an empty entity
			#[must_use]
			pub fn new() -> Self {
				Self {
					components: HashMap::new(),
				}
			}
			
			/// Creates an entity from a list of components
			#[must_use]
			pub fn from_components(components: Vec<S>) -> Self
			{
				// Create an empty entity
				let mut entity = Self::new();
				
				// Move all components into it
				for component in components {
					entity.add(component);
				}
				
				// And return it
				entity
			}
		//--------------------------------------------------------------------------------------------------
		
		// Add / Remove
		//--------------------------------------------------------------------------------------------------
			/// Adds a new component to this entity
			/// 
			/// # Return
			/// If this entity already contains a component with
			/// the same id as the one in `storage`, it is returned.
			pub fn add(&mut self, storage: S) -> Option<S>
			{
				// Insert it and remove any that already existed
				self.components.insert(storage.id(), storage)
			}
			
			/// Remove a component from this entity given it's type
			pub fn remove<C: Component<'a, S>>(&mut self) -> Option<S>
			{
				// Remove it using `remove_id`
				self.remove_id( &C::id() )
			}
			
			/// Removes a component from this entity given it's id
			pub fn remove_id(&mut self, id: &S::Id) -> Option<S>
			{
				// Attempt to remove it from it's id
				self.components.remove(id)
			}
		//--------------------------------------------------------------------------------------------------
		
		// Access
		//--------------------------------------------------------------------------------------------------
			/// Returns a reference to a component given it's type
			/// 
			/// # Return
			/// If either the component is not present in this entity
			/// or the component could not be build from the storage
			/// associated with it's id, this function will return None.
			#[must_use]
			pub fn get<C: Component<'a, S>>(&self) -> Option<&C>
			{
				// Get the storage by id and try to get the component
				// from this storage.
				self.get_id( &C::id() )
					.map(C::get)
					.flatten()
			}
			
			/// Returns a mutable reference to a component given it's type
			/// 
			/// # Return
			/// If either the component is not present in this entity
			/// or the component could not be build from the storage
			/// associated with it's id, this function will return None.
			#[must_use]
			pub fn get_mut<C: Component<'a, S>>(&mut self) -> Option<&mut C>
			{
				// Get the storage by id and try to get the component
				// from this storage.
				self.get_mut_id( &C::id() )
					.map(C::get_mut)
					.flatten()
			}
			
			/// Returns a reference to a component's storage given it's id
			#[must_use]
			pub fn get_id(&self, id: &S::Id) -> Option<&S> {
				self.components.get(id)
			}
			
			/// Returns a mutable reference to a component's storage given it's id
			#[must_use]
			pub fn get_mut_id(&mut self, id: &S::Id) -> Option<&mut S> {
				self.components.get_mut(id)
			}
		//--------------------------------------------------------------------------------------------------
		
		// Iterators
		//--------------------------------------------------------------------------------------------------
			/// Returns an iterator over all component ids in this entity
			pub fn ids(&self) -> impl Iterator<Item = &I> {
				self.components.keys()
			}
			
			/// Returns an iterator over all components in this entity
			pub fn components(&self) -> impl Iterator<Item = &S> {
				self.components.values()
			}
			
			/// Returns a mutable iterator over all components in this entity
			pub fn components_mut(&mut self) -> impl Iterator<Item = &mut S> {
				self.components.values_mut()
			}
		//--------------------------------------------------------------------------------------------------
		
		// Checks
		//--------------------------------------------------------------------------------------------------
			/// Checks if this entity has a component given it's type
			#[must_use]
			pub fn has<C: Component<'a, S>>(&self) -> bool {
				self.has_id( &C::id() )
			}
			
			/// Checks if this entity has a component given it's id
			#[must_use]
			pub fn has_id(&self, id: &S::Id) -> bool {
				self.components.contains_key(id)
			}
		//--------------------------------------------------------------------------------------------------
	}
	
	impl<'a, S, I> Default for Entity<'a, S>
	where
		S: Storage<'a, Id = I>,
		I: KeyType + 'a,
	{
		#[must_use]
		fn default() -> Self {
			Self::new()
		}
	}
	
	// Serde
	//--------------------------------------------------------------------------------------------------
		#[cfg(feature = "serde-serialize")]
		impl<'a, S> serde::Serialize for Entity<'a, S>
		where
			S    : Storage<'a> + serde::Serialize,
			S::Id: KeyType,
		{
			fn serialize<SS>(&self, serializer: SS) -> Result<SS::Ok, SS::Error>
			where
				SS: serde::Serializer,
			{
				use serde::ser::SerializeSeq;
				
				let mut seq = serializer.serialize_seq( Some(self.components.len()) )?;
				
				for component in self.components.values() {
					seq.serialize_element(component)?;
				}
				
				seq.end()
			}
		}
		
		#[cfg(feature = "serde-serialize")]
		impl<'a, 'de, S> serde::Deserialize<'de> for Entity<'a, S>
		where
			S    : Storage<'a> + serde::Deserialize<'de>,
			S::Id: KeyType,
		{
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				// Get all components as a vec
				let components = Vec::deserialize(deserializer)?;
				
				// And create the entity from all components
				Ok( Self::from_components(components) )
			}
		}
	//--------------------------------------------------------------------------------------------------
//--------------------------------------------------------------------------------------------------

