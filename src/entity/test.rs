//! Tests

// Crate
use crate as mecs;
use mecs::{Component, Entity};

// Types
//--------------------------------------------------------------------------------------------------
	// Component types
	type A = i32;
	type B = &'static str;
	type C = f32;
	
	mecs::impl_enum_storage!{
		
		/// Dummy storage type
		#[derive(PartialEq, Clone, Debug)]
		enum Components {
			A(i32),
			B(&'static str),
			C(f32),
		}
		
	}
//--------------------------------------------------------------------------------------------------

// Functions
//--------------------------------------------------------------------------------------------------
	#[test]
	fn empty()
	{
		let mut entity: Entity<Components> = Entity::new();
		
		assert_eq!(entity.get::<A>(), None);
		assert_eq!(entity.get::<B>(), None);
		assert_eq!(entity.get::<C>(), None);
		
		assert_eq!(entity.get_mut::<A>(), None);
		assert_eq!(entity.get_mut::<B>(), None);
		assert_eq!(entity.get_mut::<C>(), None);
		
		assert_eq!(entity.get_id( &<A as Component<Components>>::id() ), None);
		assert_eq!(entity.get_id( &<B as Component<Components>>::id() ), None);
		assert_eq!(entity.get_id( &<C as Component<Components>>::id() ), None);
		
		assert_eq!(entity.get_mut_id( &<A as Component<Components>>::id() ), None);
		assert_eq!(entity.get_mut_id( &<B as Component<Components>>::id() ), None);
		assert_eq!(entity.get_mut_id( &<C as Component<Components>>::id() ), None);
		
		assert_eq!(entity.ids           ().next(), None);
		assert_eq!(entity.components    ().next(), None);
		assert_eq!(entity.components_mut().next(), None);
		
		assert!(!entity.has_id( &<A as Component<Components>>::id() ));
		assert!(!entity.has_id( &<B as Component<Components>>::id() ));
		assert!(!entity.has_id( &<C as Component<Components>>::id() ));
		
		assert!(!entity.has::<A>());
		assert!(!entity.has::<B>());
		assert!(!entity.has::<C>());
	}
	
	#[test]
	fn from_components()
	{
		let mut a = Components::A(5);
		let mut b = Components::B("Hello, World!");
		let mut c = Components::C(4.5);
		
		let mut entity = mecs::entity![ a.clone(), b.clone(), c.clone() ];
		
		assert_eq!(entity.get::<A>(), Some(&5));
		assert_eq!(entity.get::<B>(), Some(&"Hello, World!"));
		assert_eq!(entity.get::<C>(), Some(&4.5));
		
		assert_eq!(entity.get_mut::<A>(), Some(&mut 5));
		assert_eq!(entity.get_mut::<B>(), Some(&mut "Hello, World!"));
		assert_eq!(entity.get_mut::<C>(), Some(&mut 4.5));
		
		assert_eq!(entity.get_id( &<A as Component<Components>>::id() ), Some(&a));
		assert_eq!(entity.get_id( &<B as Component<Components>>::id() ), Some(&b));
		assert_eq!(entity.get_id( &<C as Component<Components>>::id() ), Some(&c));
		
		assert_eq!(entity.get_mut_id( &<A as Component<Components>>::id() ), Some(&mut a));
		assert_eq!(entity.get_mut_id( &<B as Component<Components>>::id() ), Some(&mut b));
		assert_eq!(entity.get_mut_id( &<C as Component<Components>>::id() ), Some(&mut c));
		
		assert_eq!(entity.ids           ().count(), 3);
		assert_eq!(entity.components    ().count(), 3);
		assert_eq!(entity.components_mut().count(), 3);
		
		assert!(entity.has_id( &<A as Component<Components>>::id() ));
		assert!(entity.has_id( &<B as Component<Components>>::id() ));
		assert!(entity.has_id( &<C as Component<Components>>::id() ));
		
		assert!(entity.has::<A>());
		assert!(entity.has::<B>());
		assert!(entity.has::<C>());
	}
//--------------------------------------------------------------------------------------------------
