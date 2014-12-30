#![feature(phase)]
#![deny(missing_docs, warnings)]

//! Blazingly fast non-blocking IO.

#[cfg(test)]
extern crate test;

#[phase(plugin, link)]
extern crate log;

extern crate event;
extern crate mio;

