#![feature(proc_macro_non_items)]

#[macro_use] extern crate glsl_quasiquote;

#[test]
fn void_main_empty() {
  let _ = glsl!{void main() {}};
}

#[test]
fn void_main_empty_str() {
  let _ = glsl_str!{"void main() {}"};
}
