#![no_std]

//! # Upa - macro that removes the hassle of creating long pointer chains.
//! That's right, you won't see `(*(*(*(*p).a).b).c).d` stuff in your unsafe coded
//! ever again.
//! ```
//! # struct Foo {
//! #     bar: *mut Bar,
//! # }
//! #
//! # struct Bar {
//! #     quz: *mut Quz,
//! # }
//! #
//! # struct Quz {
//! #     tau: *mut Tau,
//! # }
//! #
//! # struct Tau {
//! #     val: i32,
//! # }
//! # use upa::p;
//! fn main() {
//!     let mut t = Tau { val: 1337 };
//!     let mut q = Quz { tau: &mut t };
//!     let mut b = Bar { quz: &mut q };
//!     let f: *mut Foo = &mut Foo { bar: &mut b };
//!
//!     unsafe {
//!         let wow = p!(f->bar->quz->tau->val);
//!         assert_eq!(wow, 1337);
//!     }
//! }
//! ```
//! *P.S. Identifier is the only item allowed in between `->`*.

/// Makes all of your problems go away.
#[macro_export]
macro_rules! p {
    ([$($done:tt)*] $last:ident) => {
        (*$($done)*).$last
    };
    ([ $($inner:tt)* ] $next:ident, $($tail:tt),*) => {
         p!( [ (*$($inner)*).$next ] $($tail),* )
    };
    ($pointer:ident->$($tail:tt)->*) => {
        p!( [$pointer] $($tail),* )
    };
}
