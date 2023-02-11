#![feature(const_option)]
#![feature(const_result_drop)]

#[macro_use]
extern crate num_derive;

#[cfg(test)]
#[macro_use]
pub mod regression_data_streamer;

pub mod duration;
pub mod error;
pub mod interval;
pub mod pitch;
pub mod to_lilypond;
