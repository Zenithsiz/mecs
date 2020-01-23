//! Iterators over [`World`]

// Traits
use std::iter::Iterator;

// Crate
use crate::{util::KeyType, Storage, Entity, World, EntityId};

// Types
//--------------------------------------------------------------------------------------------------
	/// Iterator over a world predicate
	#[allow(clippy::module_name_repetitions)]
	pub struct PredIter<'a, 'b, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType,
	{
		/// Reference to the world
		pub(in super) world: &'b World<'a, S>,
		
		/// The predicate id
		pub(in super) id: usize,
		
		/// The current index
		pub(in super) cur_idx: usize,
	}
	
	/// Mutable iterator over a world predicate
	#[allow(clippy::module_name_repetitions)]
	pub struct PredIterMut<'a, 'b, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType,
	{
		/// Reference to the world
		pub(in super) world: &'b mut World<'a, S>,
		
		/// The predicate id
		pub(in super) id: usize,
		
		/// The current index
		pub(in super) cur_idx: usize,
	}
//--------------------------------------------------------------------------------------------------

// Impl
//--------------------------------------------------------------------------------------------------
	impl<'a, 'b, S> Iterator for PredIter<'a, 'b, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType,
	{
		type Item = &'b Entity<'a, S>;
		
		#[allow(clippy::integer_arithmetic)] // We need to add one to get the next index
		fn next(&mut self) -> Option< Self::Item >
		{
			// While we have a next id, try to get it
			while let Some(entity_id) = self.world.predicates
				.get(&self.id)
				.expect("Could not get predicate from id")
				.ids
				.get(self.cur_idx)
			{
				// Increase the current index
				self.cur_idx += 1;
				
				// Try to get the entity
				if let Some(entity) = self.world.entities.get( &entity_id.get() )
				{
					// Transmute it to it's lifetime
					// TODO: Check if this is fine
					return Some( unsafe {
						std::mem::transmute::<&'_ _, &'b _>(entity)
					});
				}
				
				// Else set the id to null
				else {
					entity_id.set( EntityId::null() );
				}
			}
			
			// If we get here, return None
			None
		}
	}
	
	impl<'a, 'b, S> Iterator for PredIterMut<'a, 'b, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType,
	{
		type Item = &'b mut Entity<'a, S>;
		
		#[allow(clippy::integer_arithmetic)] // We need to add one to get the next index
		fn next(&mut self) -> Option< Self::Item >
		{
			// While we have a next id, try to get it
			while let Some(entity_id) = self.world.predicates
				.get_mut(&self.id)
				.expect("Could not get predicate from id")
				.ids
				.get_mut(self.cur_idx)
			{
				// Increase the current index
				self.cur_idx += 1;
				
				// Try to get the entity
				if let Some(entity) = self.world.entities.get_mut( &entity_id.get() )
				{
					// Transmute it to it's lifetime
					// TODO: Check if this is fine
					return Some( unsafe {
						std::mem::transmute::<&'_ mut _, &'b mut _>(entity)
					});
				}
				
				// Else set the id to null
				else {
					entity_id.set( EntityId::null() );
				}
			}
			
			// If we get here, return None
			None
		}
	}
//--------------------------------------------------------------------------------------------------
