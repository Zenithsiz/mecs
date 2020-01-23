//! Dynamic storage for components
//! 
//! This modules contains the [`DynStorage`] type
//! which is a storage capable of storing any type
//! that may be stored in an [Any](std::any::Any) with static lifetimes

// Traits
use std::fmt::Debug;

// Any
use std::any::{Any, TypeId};

// Crate
use crate::{Storage, Component};

// Types
//--------------------------------------------------------------------------------------------------
	/// Dynamic storage based on [`TypeId`](std::any::TypeId)
	/// for types with `'static` lifetime
	#[allow(clippy::module_name_repetitions)]
	#[derive(Debug)]
	pub struct DynStorage(TypeId, Box<dyn Any>);
//--------------------------------------------------------------------------------------------------

// Impl
//--------------------------------------------------------------------------------------------------
	impl DynStorage
	{
		// Constructors
		//--------------------------------------------------------------------------------------------------
			/// Creates a new dynamic storage from any storage component
			pub fn new<T>(value: T) -> Self
			where
				T: Component<'static, Self> + 'static
			{
				Self(T::id(), Box::new(value))
			}
		//--------------------------------------------------------------------------------------------------
		
		// Component
		//--------------------------------------------------------------------------------------------------
			/// Consumes the storage and returns the object within
			#[must_use]
			pub fn into_inner(self) -> (TypeId, Box<dyn Any>) {
				(self.0, self.1)
			}
		//--------------------------------------------------------------------------------------------------
	}
	
	impl Storage<'static> for DynStorage
	{
		type Id = TypeId;
		
		#[must_use]
		fn id(&self) -> Self::Id {
			self.0
		}
	}
	
	impl<T> Component<'static, DynStorage> for T
	where
		T: Debug + 'static
	{
		#[must_use]
		fn id() -> TypeId {
			TypeId::of::<Self>()
		}
		
		#[must_use]
		fn get(storage: &DynStorage) -> Option<&Self> {
			storage.1.downcast_ref()
		}
		
		#[must_use]
		fn get_mut(storage: &mut DynStorage) -> Option<&mut Self> {
			storage.1.downcast_mut()
		}
	}
	
	/*
	auto trait IsDynStorage {}
	impl IsDynStorage for DynStorage {}
	*/
	
	/*
	impl<T> From<T> for DynStorage
	where
		T: Component<'static, DynStorage>
	{
		fn from(value: T) -> Self {
			Self::new(value)
		}
	}
	*/
//--------------------------------------------------------------------------------------------------
