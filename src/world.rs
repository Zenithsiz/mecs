//! A world, where entities live
//! 
//! By default, all entities should live a world, so
//! that they are capable to being iterated over by
//! a system.
//! This module provides a [`World`] type to implement
//! this feature. However it should be noted that this
//! type is decoupled from all other types used, so it
//! is not required to use the ecs system.
//! However, it is reccomended, as it offers various
//! features that may help both performance and ease
//! of use.
//! 
//! # Examples
//! 
//! ```rust
//! use mecs::{World, DynStorage};
//! 
//! let mut world: World<DynStorage> = World::new();
//! 
//! let pred_id = world.add_pred(|entity| entity.has::<i32>() && entity.has::<&str>());
//! 
//! world.add( mecs::entity![ DynStorage::new(1i32), DynStorage::new("hello") ] );
//! world.add( mecs::entity![ DynStorage::new(2i32), DynStorage::new("world") ] );
//! 
//! let iter = world.iter_pred(pred_id).unwrap();
//! for entity in iter {
//! 	let num : &i32  = entity.get().unwrap();
//! 	let name: &&str = entity.get().unwrap();
//! 	
//! 	println!("{}: {}", num, name);
//! }
//! ```
//! 
//! Prints:
//! 
//! ```text
//! 1: hello
//! 2: world
//! ```

// Modules
pub mod entity_id;
pub mod iter;
    mod pred;

// Exports
pub use entity_id::EntityId;
pub use iter::{PredIter, PredIterMut};
    use pred::{Predicate, PredicateIds};

// Collections
use std::collections::HashMap;

// Traits
use std::iter::Iterator;

// Cell
use std::cell::Cell;

// Crate
use crate::{KeyType, Storage, Entity};

// Types
//--------------------------------------------------------------------------------------------------
	/// Collection of entities
	/// 
	/// # Id
	/// Each entity has it's own unique id, that will
	/// not be re-used even after the entity is removed
	/// from this world.
	/// The entity id 0 is reserved to mean a null entity,
	/// that is, an entity which does not exist.
	pub struct World<'a, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType,
	{
		/// All of the entities
		pub(in self) entities: HashMap<EntityId, Entity<'a, S>>,
		
		/// The next id to use
		next_entity_id: EntityId,
		
		
		/// All predicates
		pub(in self) predicates: HashMap<usize, PredicateIds<'a, S>>,
		
		/// Next predicate id to use
		next_pred_id: usize,
	}
//--------------------------------------------------------------------------------------------------

// Impl
//--------------------------------------------------------------------------------------------------
	impl<'a, S> World<'a, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType,
	{
		// Constructors
		//--------------------------------------------------------------------------------------------------
			/// Create a new empty world
			#[must_use]
			pub fn new() -> Self
			{
				Self {
					entities: HashMap::new(),
					next_entity_id : EntityId::new(1),
					
					predicates: HashMap::new(),
					next_pred_id: 1,
				}
			}
			
			/// Creates a world from a list of entities
			#[must_use]
			pub fn from_entities(entities: Vec< Entity<'a, S> >) -> Self
			{
				// Create an empty world
				let mut world = Self::new();
				
				// Move all entities into it
				for entity in entities {
					world.add(entity);
				}
				
				// And return it
				world
			}
		//--------------------------------------------------------------------------------------------------
		
		// Add / Remove
		//--------------------------------------------------------------------------------------------------
			/// Adds an entity to this world
			/// 
			/// # Return
			/// This function returns the id associated with the
			/// entity inserted. Usually this id is not used, as
			/// iteration through the world is done using either
			/// the predicates, or with global iteration.
			/// 
			/// Currently removal of entities is only supported
			/// through id, but in the future, it will be added
			/// a feature to remove entities during iteration as
			/// well.
			pub fn add(&mut self, entity: Entity<'a, S>) -> EntityId
			{
				// Get the id to use for this entity
				// and increase the next id
				let id = self.next_entity_id;
				self.next_entity_id.inc();
				
				// Update each predicate
				for PredicateIds { pred, ids } in self.predicates.values_mut()
				{
					// Get rid of any nulls in-place
					ids.retain(|id| !id.get().is_null());
					
					// And add the id we have
					if pred(&entity) {
						ids.push( Cell::new(id) )
					}
				}
				
				// Insert the entity
				self.entities.insert(id, entity);
				
				// And return it's id
				id
			}
			
			/// Removes an entity from this world given it's id
			/// 
			/// # Ids
			/// Currently this function is the only way to remove entities
			/// from the world, in the future a feature will be added to
			/// remove them during iteration.
			pub fn remove(&mut self, id: EntityId) -> Option< Entity<'a, S> >
			{
				// And remove the entity
				self.entities.remove(&id)
			}
		//--------------------------------------------------------------------------------------------------
		
		// Access
		//--------------------------------------------------------------------------------------------------
			/// Returns a reference to an entity given it's id
			#[must_use]
			pub fn get(&self, id: EntityId) -> Option<&Entity<'a, S>> {
				self.entities.get(&id)
			}
			
			/// Returns a mutable reference to an entity given it's id
			#[must_use]
			pub fn get_mut(&mut self, id: EntityId) -> Option<&mut Entity<'a, S>> {
				self.entities.get_mut(&id)
			}
		//--------------------------------------------------------------------------------------------------
		
		// Register
		//--------------------------------------------------------------------------------------------------
			/// Registers a predicate to filter entities through
			#[allow(clippy::integer_arithmetic)] // We need to add one to get the next pred id
			pub fn add_pred<F>(&mut self, f: F) -> usize
			where
				F: Fn(&Entity<'a, S>) -> bool + 'static
			{
				// Get the id to use for this predicate
				// and increase the next id
				let id = self.next_pred_id;
				self.next_pred_id += 1;
				
				// Go through all entities and add the ones that match this predicate
				let mut ids = vec![];
				for (&entity_id, entity) in &self.entities
				{
					if f(entity) {
						ids.push( Cell::new(entity_id) );
					}
				}
				
				// Insert the predicate
				self.predicates.insert(id, PredicateIds{ pred: Predicate( Box::new(f) ), ids });
				
				// And return it's id
				id
			}
		//--------------------------------------------------------------------------------------------------
		
		// Iterators
		//--------------------------------------------------------------------------------------------------
			/// Returns an iterator over all entities in this world
			pub fn iter_all(&self) -> impl Iterator<Item = &Entity<'a, S>> {
				self.entities.values()
			}
			
			/// Returns a mutable iterator over all entities in this world
			pub fn iter_mut_all(&mut self) -> impl Iterator<Item = &mut Entity<'a, S>> {
				self.entities.values_mut()
			}
			
			/// Returns an iterator over a predicate
			#[must_use]
			pub fn iter_pred(&self, id: usize) -> Option< PredIter<'a, '_, S> > {
				if self.predicates.get(&id).is_some() {
					Some( PredIter {
							world: self,
							id,
							cur_idx: 0,
					})
				} else {
					None
				}
			}
			
			/// Returns a mutable iterator over a predicate
			#[must_use]
			pub fn iter_pred_mut(&mut self, id: usize) -> Option< PredIterMut<'a, '_, S> > {
				if self.predicates.get(&id).is_some() {
					Some( PredIterMut {
							world: self,
							id,
							cur_idx: 0,
					})
				} else {
					None
				}
			}
		//--------------------------------------------------------------------------------------------------
	}
	
	impl<'a, S> Default for World<'a, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType,
	{
		#[must_use]
		fn default() -> Self {
			Self::new()
		}
	}
	
	impl<'a, S> PartialEq for World<'a, S>
	where
		S    : Storage<'a> + PartialEq,
		S::Id: KeyType,
	{
		#[must_use]
		fn eq(&self, other: &Self) -> bool
		{
			// Compare just the entities
			self.entities == other.entities
		}
	}
	
	impl<'a, S> Eq for World<'a, S>
	where
		S    : Storage<'a> + Eq,
		S::Id: KeyType,
	{}
	
	// Serde
	//--------------------------------------------------------------------------------------------------
		#[cfg(feature = "serde-serialize")]
		impl<'a, S> serde::Serialize for World<'a, S>
		where
			S    : Storage<'a> + serde::Serialize,
			S::Id: KeyType,
		{
			fn serialize<SS>(&self, serializer: SS) -> Result<SS::Ok, SS::Error>
			where
				SS: serde::Serializer,
			{
				use serde::ser::SerializeSeq;
				
				let mut seq = serializer.serialize_seq( Some(self.entities.len()) )?;
				
				for entity in self.entities.values() {
					seq.serialize_element(entity)?;
				}
				
				seq.end()
			}
		}
		
		#[cfg(feature = "serde-serialize")]
		impl<'a, 'de, S> serde::Deserialize<'de> for World<'a, S>
		where
			S    : Storage<'a> + serde::Deserialize<'de>,
			S::Id: KeyType,
		{
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				// Get all entities as a vec
				let entities = Vec::deserialize(deserializer)?;
				
				// And create the world from all entities
				Ok( Self::from_entities(entities) )
			}
		}
	//--------------------------------------------------------------------------------------------------
//--------------------------------------------------------------------------------------------------
