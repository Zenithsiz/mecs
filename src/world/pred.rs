//! Predicates over a world

// Cell
use std::cell::Cell;

// Crate
use crate::{util::KeyType, Storage, Entity, EntityId};

// Types
//--------------------------------------------------------------------------------------------------
	/// A predicate
	pub struct Predicate<'a, S>( pub(in super) Box<dyn Fn(&Entity<'a, S>) -> bool> )
	where
		S    : Storage<'a>,
		S::Id: KeyType;
	
	/// A predicate along with it's current ids
	#[derive(Debug)]
	pub struct PredicateIds<'a, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType
	{
		/// The predicate of these ids
		pub(in super) pred: Predicate<'a, S>,
		
		/// The current ids
		pub(in super) ids: Vec< Cell<EntityId> >,
	}
//--------------------------------------------------------------------------------------------------

// Impl
//--------------------------------------------------------------------------------------------------
	impl<'a, S> std::fmt::Debug for Predicate<'a, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType,
	{
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "Predicate")
		}
	}
	
	impl<'a, S> FnOnce<(&Entity<'a, S>,)> for Predicate<'a, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType,
	{
		type Output = bool;
		
		extern "rust-call" fn call_once(self, args: (&Entity<'a, S>,)) -> Self::Output {
			self.0.call_once(args)
		}
	}
	
	impl<'a, S> FnMut<(&Entity<'a, S>,)> for Predicate<'a, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType,
	{
		extern "rust-call" fn call_mut(&mut self, args: (&Entity<'a, S>,)) -> Self::Output {
			self.0.call_mut(args)
		}
	}
	
	impl<'a, S> Fn<(&Entity<'a, S>,)> for Predicate<'a, S>
	where
		S    : Storage<'a>,
		S::Id: KeyType,
	{
		extern "rust-call" fn call(&self, args: (&Entity<'a, S>,)) -> Self::Output {
			self.0.call(args)
		}
	}
//--------------------------------------------------------------------------------------------------
