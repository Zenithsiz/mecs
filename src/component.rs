//! Components and their storage
//! 
//! All components live within a storage, which may be
//! an enum, a boxed trait, etc..., anything that implements
//! the [`Storage`] trait.
//! All types that can live within a storage must implement
//! [`Component`] of that storage.
//!
//! # Example
//! 
//! TODO: Make this work
//! ```rust,ignore
//! 
//! mecs::impl_enum_storage! {
//! 	enum Components<'a> {
//! 		A(i32),
//! 		B(&'a str),
//! 	}
//! }
//! ```
//! 
//! # Manual Implementation
//! 
//! ```rust
//! use std::any::Any;
//! use mecs::{Storage, Component};
//! 
//! enum Components<'a> {
//! 	A(i32),
//! 	B(&'a str),
//! }
//! 
//! impl<'a> Storage<'a> for Components<'a>
//! {
//! 	type Id = usize;
//! 	
//! 	fn id(&self) -> Self::Id {
//! 		match self {
//! 			Self::A(_) => <i32     as Component<Self>>::id(),
//! 			Self::B(_) => <&'a str as Component<Self>>::id(),
//! 		}
//! 	}
//! }
//! 
//! impl<'a> Component<'a, Components<'a>> for i32 {
//! 	fn id() -> <Components<'a> as Storage<'a>>::Id { 0 }
//! 	
//! 	fn get<'b>(storage: &'b Components<'a>) -> Option<&'b Self> {
//! 		if let Components::A(num) = storage { Some( num ) } else { None }
//! 	}
//! 	
//! 	fn get_mut<'b>(storage: &'b mut Components<'a>) -> Option<&'b mut Self> {
//! 		if let Components::A(num) = storage { Some( num ) } else { None }
//! 	}
//! }
//! 
//! impl<'a> Component<'a, Components<'a>> for &'a str {
//! 	fn id() -> <Components<'a> as Storage<'a>>::Id { 1 }
//! 	
//! 	fn get<'b>(storage: &'b Components<'a>) -> Option<&'b Self> {
//! 		if let Components::B(name) = storage { Some( name ) } else { None }
//! 	}
//! 	
//! 	fn get_mut<'b>(storage: &'b mut Components<'a>) -> Option<&'b mut Self> {
//! 		if let Components::B(name) = storage { Some( name ) } else { None }
//! 	}
//! }
//! ```

// Modules
#[cfg(test)]
    mod test;
    mod impl_macro;
pub mod dyn_storage;

// Exports
pub use dyn_storage::DynStorage;

// Traits
//--------------------------------------------------------------------------------------------------
	/// Storage for any number of components
	pub trait Storage<'a>
	{
		/// The type representing a unique id for each component
		/// within this storage.
		type Id;
		
		/// Returns the id of the current component within this
		/// storage.
		#[must_use]
		fn id(&self) -> Self::Id;
	}
	
	/// Trait implemented by all types within a storage.
	/// 
	/// # Visitor
	/// This trait uses the visitor pattern to provide getters for
	/// the storage.
	pub trait Component<'a, S>
	where
		S: Storage<'a>,
	{
		/// Returns this component's id
		#[must_use]
		fn id() -> S::Id;
		
		/// Returns a reference to this component from a storage
		#[must_use]
		fn get(storage: &S) -> Option<&Self>;
		
		/// Returns a mutable reference to this component from a storage
		#[must_use]
		fn get_mut(storage: &mut S) -> Option<&mut Self>;
	}
//--------------------------------------------------------------------------------------------------
