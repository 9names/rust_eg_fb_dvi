#!/bin/sh
# todo: use cbindgen in build.rs
# see https://github.com/eqrion/cbindgen/blob/master/docs.md
cargo build
cbindgen --lang c --output rust-eg-fb.h 