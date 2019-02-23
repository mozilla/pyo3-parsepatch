#![feature(specialization)]

extern crate parsepatch;

#[macro_use]
extern crate pyo3;

pub mod common;
pub mod counts;
pub mod diffs;
pub mod init;
pub mod lines;
