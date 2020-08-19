# Wingspan Hacking Guide

First of all, thanks for taking the time to read this. Currently this is just a
list of things to note, but one day I'll write a good guide

## Things to note

- Don't block the UI thread
- Don't run `cargo fmt`, as the rules vendored stuff are different to the rules for wingspan stuff.
- Instead use `cargo fm`, which will only do the stuff you want it to
- Same with `cargo clippy`, use `cargo cl` 