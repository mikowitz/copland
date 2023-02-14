#![feature(const_option)]
#![feature(const_result_drop)]
#![feature(associated_type_defaults)]

#[macro_use]
extern crate num_derive;

#[cfg(test)]
#[macro_use]
pub mod regression_data_streamer;

pub mod chord;
pub mod duration;
pub mod error;
pub mod interval;
pub mod leaf;
pub mod note;
pub mod notehead;
pub mod pitch;
pub mod rest;
pub mod spacer;
pub mod to_lilypond;
