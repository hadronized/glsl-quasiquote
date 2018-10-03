#![feature(proc_macro_non_items)]

#[macro_use] extern crate glsl_quasiquote;

#[test]
fn void_main_empty() {
  let _ = glsl!{"void main() {}"};
}
