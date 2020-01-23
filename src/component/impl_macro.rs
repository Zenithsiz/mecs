//! Macros for implementing storage with ease

// Macros
//--------------------------------------------------------------------------------------------------
	/// Creates and implements [`Storage`] and [`Component`] for an enum with all possible component types
	/// using an incremental `u64` for the id
	#[macro_export]
	macro_rules! impl_enum_storage
	{
		// Component
		//--------------------------------------------------------------------------------------------------
			// Tail
			{@ComponentImpl
				$cur_idx: expr,
				$name   : ident,
			} => {};
			
			// Main Branch
			{@ComponentImpl
				$cur_idx  : expr ,
				$name     : ident,
				
				$variant_name: ident ($variant_type: ty),
				
				$($variant_names: ident ($variant_types: ty),)*
			} =>
			{
				// Impl the head type with the current index
				impl<'a> $crate::Component<'a, $name> for $variant_type {
					#[must_use]
					fn id() -> <$name as $crate::Storage<'a>>::Id {
						$cur_idx
					}
					
					#[must_use]
					fn get(storage: &$name) -> Option<&Self> {
						if let $name::$variant_name(value) = storage { Some(value) }
						else                                         { None        }
					}
					
					#[must_use]
					fn get_mut(storage: &mut $name) -> Option<&mut Self> {
						if let $name::$variant_name(value) = storage { Some(value) }
						else                                         { None        }
					}
				}
				
				// Implement all types within the tail
				$crate::impl_enum_storage!{@ComponentImpl
					$cur_idx + 1,
					$name,
					$($variant_names($variant_types),)*
				}
			};
			
			// Entry point
			{@ComponentImpl
				$name: ident,
				
				$($variant_name: ident ($variant_type: ty),)*
			} =>
			{
				$crate::impl_enum_storage!{@ComponentImpl
					0,
					$name,
					$($variant_name($variant_type),)*
				}
			};
		//--------------------------------------------------------------------------------------------------
		
		// Component
		//--------------------------------------------------------------------------------------------------
			// Entry point
			{@StorageImpl
				// Enum name
				$name: ident,
				
				// Variants
				$(
					$( #[$variant_meta: meta] )*
					$variant_name: ident( $variant_type: ty )
				),*
			} =>
			{
				impl<'a> $crate::Storage<'a> for $name
				{
					type Id = u64;
					
					#[must_use]
					fn id(&self) -> Self::Id {
						match self {
							$(
								Self::$variant_name(_) => { <$variant_type as $crate::Component<$name>>::id() }
							)*
						}
					}
				}
			};
		//--------------------------------------------------------------------------------------------------
		
		// Main Entry Point
		{
			// Enum declaration
			$( #[$enum_meta:meta] )*
			$vis:vis enum $name:ident
			{
				$(
					$( #[$variant_meta: meta] )*
					$variant_name: ident ( $variant_type: ty )
				),*
				
				$(,)?
			}
		} =>
		{
			// Enum declaration
			$( #[$enum_meta] )*
			$vis enum $name
			{
				$(
					$( #[$variant_meta] )*
					$variant_name( $variant_type ),
				)*
			}
			
			$crate::impl_enum_storage!(@StorageImpl
				$name,
				$(
					$( #[$variant_meta] )*
					$variant_name( $variant_type )
				),*
			);
			
			$crate::impl_enum_storage!(@ComponentImpl $name, $( $variant_name($variant_type), )*);
		}
	}
//--------------------------------------------------------------------------------------------------
