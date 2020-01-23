//! Utilities

// Traits
use std::hash::Hash;

// Traits
//--------------------------------------------------------------------------------------------------
	/// A key type for [`HashMap`]
	pub trait KeyType: PartialEq + Eq + Hash {}
//--------------------------------------------------------------------------------------------------

// Impl
//--------------------------------------------------------------------------------------------------
	impl<T> KeyType for T
	where
		T: ?Sized + PartialEq + Eq + Hash
	{}
//--------------------------------------------------------------------------------------------------
