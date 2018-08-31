#![warn(warnings)]

extern crate cobalt;
extern crate itertools;
extern crate liquid;
extern crate normalize_line_endings;
extern crate regex;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

pub use error::Error;

pub mod error;

mod jekyll;
pub mod jekyll_model;
pub use jekyll::*;
