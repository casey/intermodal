//! This crate contains Intermodal specifications.
//!
//! It is written as a Rust crate, instead of conventional markup, so that
//! included Rust code can be checked for correctness, and to allow rustdoc to
//! generate inter-item links.
//!
//! All public items are marked as such solely so they may be used in doc tests.
//! They may be changed or removed at any time, with or without a version bump.

#![feature(const_generics)]
#![allow(incomplete_features)]
#![deny(missing_docs)]

pub mod archive;
