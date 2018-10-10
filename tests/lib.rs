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

#[test]
fn understands_version() {
  let _ = glsl_str!{"
    #version 330 core
    void main() {
    }
  "};
}

#[test]
fn fn_returns_int() {
  let _ = glsl!{
    int test() {
      return 3.;
    }
  };
}

#[test]
fn simple_struct() {
  let _ = glsl!{
    struct V {
      vec4 p;
      vec2 uv;
    };

    struct F {
      vec4 color;
    };
  };
}

#[test]
fn struct_several_ident_per_field() {
  let _ = glsl!{
    struct S {
      float a, b, c;
    };
  };
}

#[test]
fn struct_with_identifiers() {
  let _ = glsl!{
    struct S {
      float a, b, c;
    } foo, bar, zoo;
  };
}

#[test]
fn struct_with_arrayed_identifiers() {
  let _ = glsl!{
    struct S {
      float a, b, c;
    } foo[3], bar[12], zoo[];
  };
}
