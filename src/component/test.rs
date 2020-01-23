//! Tests

// Crate
use crate as mecs;
use mecs::{Storage, Component};

// Types
//--------------------------------------------------------------------------------------------------
	// Component types
	type A = i32;
	type B = &'static str;
	type C = f32;
	
	mecs::impl_enum_storage!{
		
		/// Dummy storage type
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
	fn macro_gen()
	{
		let mut a = Components::A(5);
		let mut b = Components::B("Hello, World!");
		let mut c = Components::C(4.5);
		
		assert_eq!(a.id(), <A as Component<Components>>::id());
		assert_eq!(b.id(), <B as Component<Components>>::id());
		assert_eq!(c.id(), <C as Component<Components>>::id());
		
		assert_ne!(a.id(), b.id());
		assert_ne!(b.id(), c.id());
		assert_ne!(a.id(), c.id());
		
		assert_eq!(A::get(&a), Some(&5));
		assert_eq!(B::get(&a), None    );
		assert_eq!(C::get(&a), None    );
		
		assert_eq!(A::get(&b), None                  );
		assert_eq!(B::get(&b), Some(&"Hello, World!"));
		assert_eq!(C::get(&b), None                  );
		
		assert_eq!(A::get(&c), None      );
		assert_eq!(B::get(&c), None      );
		assert_eq!(C::get(&c), Some(&4.5));
		
		*A::get_mut(&mut a).unwrap() = 8;
		*B::get_mut(&mut b).unwrap() = "Bye, World!";
		*C::get_mut(&mut c).unwrap() = 2.3;
		
		assert_eq!(A::get(&a), Some(&8            ));
		assert_eq!(B::get(&b), Some(&"Bye, World!"));
		assert_eq!(C::get(&c), Some(&2.3          ));
	}
//--------------------------------------------------------------------------------------------------
