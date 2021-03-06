#![no_std]
#![cfg_attr(all(feature = "use_alloc", not(feature = "use_std")), feature(alloc))]
#![cfg_attr(all(feature = "use_collections", not(feature = "use_std")), feature(collections))]

//! A library that provides a way to logically own objects, whether or not
//! heap allocation is available.

#[cfg(feature = "use_std")]
extern crate std;
#[cfg(all(feature = "use_alloc", not(feature = "use_std")))]
extern crate alloc;
#[cfg(all(feature = "use_collections", not(feature = "use_std")))]
extern crate collections;

mod object;
mod slice;

pub use object::Managed;
pub use slice::ManagedSlice;
