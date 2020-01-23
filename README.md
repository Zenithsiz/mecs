# MEcs

Ecs library with a focus on iteration performance

# Example

The following is a small example on how to use the ecs library:

```rust
use mecs::{World};

/// Position component
#[derive(PartialEq, Clone, Copy, Debug)]
struct Position(pub f32, pub f32);

/// Velocity component
#[derive(PartialEq, Clone, Copy, Debug)]
struct Velocity(pub f32, pub f32);

mecs::impl_enum_storage! {
	/// All of our components
	enum Components {
		Position(Position),
		Velocity(Velocity),
	}
}

fn main()
{
	let mut world: World<Components> = World::new();
	
	world.add( mecs::entity![
		Components::Position( Position(1.0, 10.0) ),
		Components::Velocity( Velocity(0.1, -1.0) ),
	]);
	
	let pred_id = world.add_pred(|entity| entity.has::<Position>() && entity.has::<Velocity>());
	
	for i in 0..10 {
		for entity in world.iter_pred_mut(pred_id).unwrap()
		{
			let vel:      Velocity = *entity.get    ().unwrap();
			let pos: &mut Position =  entity.get_mut().unwrap();
			
			pos.0 += vel.0;
			pos.1 += vel.1;
			
			println!("{:?}", pos);
		}
	}
}
```

Will print

```text
Position { 1.1, 9.0 }
Position { 1.2, 8.0 }
Position { 1.3, 7.0 }
...
Position { 2.0, 0.0 }
```

# Nightly
This library currently uses features only available on the nightly channel, particularly the `Fn` family of traits.
In the future, once these are stabilized, the library will be able to be used on stable.
