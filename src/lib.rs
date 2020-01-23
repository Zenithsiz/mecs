//! Entity component system implementation

// Features
#![feature(unboxed_closures    )]
#![feature(fn_traits           )]

// Warnings
//--------------------------------------------------------------------------------------------------
	// Use all warnings from clippy
	#![warn(
		clippy::all,
		clippy::restriction,
		clippy::pedantic,
		clippy::nursery,
		clippy::cargo,
	)]
	
	// LTO
	// TODO: Check when we can remove this
	#![allow(clippy::missing_inline_in_public_items)]
	
	// Style
	#![allow(clippy::implicit_return   )] // Our convention uses tail returns
	#![allow(clippy::panic             )] // We use panic when we reach an unstable state
	#![allow(clippy::result_expect_used)] // We use expect when making assertions
	#![allow(clippy::option_expect_used)] // ...
	#![allow(clippy::indexing_slicing  )] // We use [] when we want to panic if the value doesn't exist
	#![allow(clippy::unreachable       )] // We use unreachable when making assertions
//--------------------------------------------------------------------------------------------------



// Modules
    mod util;
pub mod component;
pub mod entity;
pub mod world;

// Exports
    use util     ::KeyType;
pub use component::{Component, Storage, DynStorage};
pub use entity   ::Entity;
pub use world    ::{World, EntityId};
