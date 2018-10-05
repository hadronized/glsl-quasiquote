//! # GLSL quasiquoting.
//!
//! This crate exports two procedural macros: `glsl!` and `glsl_str!`. They enable quasiquoting by
//! allowing you to embed GLSL source code directly into rust via the syntax:
//!
//! ```ignore
//! glsl!{
//!   // your GLSL code here
//!   void main() {
//!   }
//! }
//! ```
//!
//! The `glsl!` macro accepts the GLSL code directly. You should be using that macro in pretty much
//! all situations, but there’s an edge corner that might require you to use its `glsl_str!`
//! sibling: if you wan to use the `#version` or `#extension` GLSL pragmas. Rust procedural macro
//! system will parse those pragmas as regular Rust token and will ignore the mandatory `\n`,
//! causing the macro to fail. In that case, you need to use an opaque string to encode the newlines
//! by doing so:
//!
//! ```ignore
//! glsl_str!{"
//!   #version 330 core
//!   // your GLSL code here
//! "}
//! ```
//!
//! Both the `glsl!` and `glsl_str!` procedural macro resolve at compile-time to
//! `glsl::syntax::TranslationUnit`, allowing you to manipulate the GLSL AST directly. Feel free
//! to have a look at the [`glsl`](https://crates.io/crates/glsl) crate for further information.
//!
//! # Getting started
//! 
//! Add the following to your dependencies in your `Cargo.toml`:
//! 
//! ```
//! glsl = "0.9"
//! glsl-quasiquote = "0.1"
//! ```
//! 
//! Then, you currently need to have a nightly compiler and the following feature enabled:
//! 
//! ```
//! #![feature(proc_macro_non_items)]
//! ```
//! 
//! Then, depending on which you’re using the 2018 edition or not:
//! 
//! > *Non-2018 edition*
//! 
//! ```
//! extern crate glsl;
//! #[macro_use] extern crate glsl_quasiquote;
//! ```
//! 
//! > *2018 edition*
//! 
//! ```
//! use glsl_quasiquote::{glsl, glsl_str};
//! ```

extern crate glsl;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use] extern crate quote;

mod quoted_option;

use glsl::parser::{ParseResult, parse_str};
use glsl::parsers::translation_unit;
use glsl::syntax;
use proc_macro2::TokenStream;

use quoted_option::QuotedOption;

#[proc_macro]
pub fn glsl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let s = format!("{}", input);
  let parsed = parse_str(s.as_str(), translation_unit);

  if let ParseResult::Ok(tu) = parsed {
    tokenize_translation_unit(&tu).into()
  } else {
    panic!("GLSL error: {:?}", parsed);
  }
}

#[proc_macro]
pub fn glsl_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  // we assume only one token: a string
  match input.into_iter().next() {
    Some(proc_macro::TokenTree::Literal(ref input_str, ..)) => {
      let s = format!("{}", input_str);
      let s2 = &s[1..s.len()-1];
      let parsed = parse_str(s2, translation_unit);

      if let ParseResult::Ok(tu) = parsed {
        tokenize_translation_unit(&tu).into()
      } else {
        panic!("GLSL error: {:?}", parsed);
      }
    }

    x => {
      panic!("GLSL error: incorrect macro invocation, please use a single opaque string; saw {:?}", x);
    }
  }
}

fn tokenize_type_specifier_non_array(t: &syntax::TypeSpecifierNonArray) -> TokenStream {
  match *t {
    syntax::TypeSpecifierNonArray::Void => quote!{ glsl::syntax::TypeSpecifierNonArray::Void },
    syntax::TypeSpecifierNonArray::Bool => quote!{ glsl::syntax::TypeSpecifierNonArray::Bool },
    syntax::TypeSpecifierNonArray::Int => quote!{ glsl::syntax::TypeSpecifierNonArray::Int },
    syntax::TypeSpecifierNonArray::UInt => quote!{ glsl::syntax::TypeSpecifierNonArray::UInt },
    syntax::TypeSpecifierNonArray::Float => quote!{ glsl::syntax::TypeSpecifierNonArray::Float },
    syntax::TypeSpecifierNonArray::Double => quote!{ glsl::syntax::TypeSpecifierNonArray::Double },
    syntax::TypeSpecifierNonArray::Vec2 => quote!{ glsl::syntax::TypeSpecifierNonArray::Vec2 },
    syntax::TypeSpecifierNonArray::Vec3 => quote!{ glsl::syntax::TypeSpecifierNonArray::Vec3 },
    syntax::TypeSpecifierNonArray::Vec4 => quote!{ glsl::syntax::TypeSpecifierNonArray::Vec4 },
    syntax::TypeSpecifierNonArray::DVec2 => quote!{ glsl::syntax::TypeSpecifierNonArray::DVec2 },
    syntax::TypeSpecifierNonArray::DVec3 => quote!{ glsl::syntax::TypeSpecifierNonArray::DVec3 },
    syntax::TypeSpecifierNonArray::DVec4 => quote!{ glsl::syntax::TypeSpecifierNonArray::DVec4 },
    syntax::TypeSpecifierNonArray::BVec2 => quote!{ glsl::syntax::TypeSpecifierNonArray::BVec2 },
    syntax::TypeSpecifierNonArray::BVec3 => quote!{ glsl::syntax::TypeSpecifierNonArray::BVec3 },
    syntax::TypeSpecifierNonArray::BVec4 => quote!{ glsl::syntax::TypeSpecifierNonArray::BVec4 },
    syntax::TypeSpecifierNonArray::IVec2 => quote!{ glsl::syntax::TypeSpecifierNonArray::IVec2 },
    syntax::TypeSpecifierNonArray::IVec3 => quote!{ glsl::syntax::TypeSpecifierNonArray::IVec3 },
    syntax::TypeSpecifierNonArray::IVec4 => quote!{ glsl::syntax::TypeSpecifierNonArray::IVec4 },
    syntax::TypeSpecifierNonArray::UVec2 => quote!{ glsl::syntax::TypeSpecifierNonArray::UVec2 },
    syntax::TypeSpecifierNonArray::UVec3 => quote!{ glsl::syntax::TypeSpecifierNonArray::UVec3 },
    syntax::TypeSpecifierNonArray::UVec4 => quote!{ glsl::syntax::TypeSpecifierNonArray::UVec4 },
    syntax::TypeSpecifierNonArray::Mat2 => quote!{ glsl::syntax::TypeSpecifierNonArray::Mat2 },
    syntax::TypeSpecifierNonArray::Mat3 => quote!{ glsl::syntax::TypeSpecifierNonArray::Mat3 },
    syntax::TypeSpecifierNonArray::Mat4 => quote!{ glsl::syntax::TypeSpecifierNonArray::Mat4 },
    syntax::TypeSpecifierNonArray::Mat23 => quote!{ glsl::syntax::TypeSpecifierNonArray::Mat23 },
    syntax::TypeSpecifierNonArray::Mat24 => quote!{ glsl::syntax::TypeSpecifierNonArray::Mat24 },
    syntax::TypeSpecifierNonArray::Mat32 => quote!{ glsl::syntax::TypeSpecifierNonArray::Mat32 },
    syntax::TypeSpecifierNonArray::Mat34 => quote!{ glsl::syntax::TypeSpecifierNonArray::Mat34 },
    syntax::TypeSpecifierNonArray::Mat42 => quote!{ glsl::syntax::TypeSpecifierNonArray::Mat42 },
    syntax::TypeSpecifierNonArray::Mat43 => quote!{ glsl::syntax::TypeSpecifierNonArray::Mat43 },
    syntax::TypeSpecifierNonArray::DMat2 => quote!{ glsl::syntax::TypeSpecifierNonArray::DMat2 },
    syntax::TypeSpecifierNonArray::DMat3 => quote!{ glsl::syntax::TypeSpecifierNonArray::DMat3 },
    syntax::TypeSpecifierNonArray::DMat4 => quote!{ glsl::syntax::TypeSpecifierNonArray::DMat4 },
    syntax::TypeSpecifierNonArray::DMat23 => quote!{ glsl::syntax::TypeSpecifierNonArray::DMat23 },
    syntax::TypeSpecifierNonArray::DMat24 => quote!{ glsl::syntax::TypeSpecifierNonArray::DMat24 },
    syntax::TypeSpecifierNonArray::DMat32 => quote!{ glsl::syntax::TypeSpecifierNonArray::DMat32 },
    syntax::TypeSpecifierNonArray::DMat34 => quote!{ glsl::syntax::TypeSpecifierNonArray::DMat34 },
    syntax::TypeSpecifierNonArray::DMat42 => quote!{ glsl::syntax::TypeSpecifierNonArray::DMat42 },
    syntax::TypeSpecifierNonArray::DMat43 => quote!{ glsl::syntax::TypeSpecifierNonArray::DMat43 },
    syntax::TypeSpecifierNonArray::Sampler1D => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler1D },
    syntax::TypeSpecifierNonArray::Image1D => quote!{ glsl::syntax::TypeSpecifierNonArray::Image1D },
    syntax::TypeSpecifierNonArray::Sampler2D => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler2D },
    syntax::TypeSpecifierNonArray::Image2D => quote!{ glsl::syntax::TypeSpecifierNonArray::Image2D },
    syntax::TypeSpecifierNonArray::Sampler3D => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler3D },
    syntax::TypeSpecifierNonArray::Image3D => quote!{ glsl::syntax::TypeSpecifierNonArray::Image3D },
    syntax::TypeSpecifierNonArray::SamplerCube => quote!{ glsl::syntax::TypeSpecifierNonArray::SamplerCube },
    syntax::TypeSpecifierNonArray::ImageCube => quote!{ glsl::syntax::TypeSpecifierNonArray::ImageCube },
    syntax::TypeSpecifierNonArray::Sampler2DRect => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler2DRect },
    syntax::TypeSpecifierNonArray::Image2DRect => quote!{ glsl::syntax::TypeSpecifierNonArray::Image2DRect },
    syntax::TypeSpecifierNonArray::Sampler1DArray => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler1DArray },
    syntax::TypeSpecifierNonArray::Image1DArray => quote!{ glsl::syntax::TypeSpecifierNonArray::Image1DArray },
    syntax::TypeSpecifierNonArray::Sampler2DArray => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler2DArray },
    syntax::TypeSpecifierNonArray::Image2DArray => quote!{ glsl::syntax::TypeSpecifierNonArray::Image2DArray },
    syntax::TypeSpecifierNonArray::SamplerBuffer => quote!{ glsl::syntax::TypeSpecifierNonArray::SamplerBuffer },
    syntax::TypeSpecifierNonArray::ImageBuffer => quote!{ glsl::syntax::TypeSpecifierNonArray::ImageBuffer },
    syntax::TypeSpecifierNonArray::Sampler2DMS => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler2DMS },
    syntax::TypeSpecifierNonArray::Image2DMS => quote!{ glsl::syntax::TypeSpecifierNonArray::Image2DMS },
    syntax::TypeSpecifierNonArray::Sampler2DMSArray => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler2DMSArray },
    syntax::TypeSpecifierNonArray::Image2DMSArray => quote!{ glsl::syntax::TypeSpecifierNonArray::Image2DMSArray },
    syntax::TypeSpecifierNonArray::SamplerCubeArray => quote!{ glsl::syntax::TypeSpecifierNonArray::SamplerCubeArray },
    syntax::TypeSpecifierNonArray::ImageCubeArray => quote!{ glsl::syntax::TypeSpecifierNonArray::ImageCubeArray },
    syntax::TypeSpecifierNonArray::Sampler1DShadow => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler1DShadow },
    syntax::TypeSpecifierNonArray::Sampler2DShadow => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler2DShadow },
    syntax::TypeSpecifierNonArray::Sampler2DRectShadow => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler2DRectShadow },
    syntax::TypeSpecifierNonArray::Sampler1DArrayShadow => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler1DArrayShadow },
    syntax::TypeSpecifierNonArray::Sampler2DArrayShadow => quote!{ glsl::syntax::TypeSpecifierNonArray::Sampler2DArrayShadow },
    syntax::TypeSpecifierNonArray::SamplerCubeShadow => quote!{ glsl::syntax::TypeSpecifierNonArray::SamplerCubeShadow },
    syntax::TypeSpecifierNonArray::SamplerCubeArrayShadow => quote!{ glsl::syntax::TypeSpecifierNonArray::SamplerCubeArrayShadow },
    syntax::TypeSpecifierNonArray::ISampler1D => quote!{ glsl::syntax::TypeSpecifierNonArray::ISampler1D },
    syntax::TypeSpecifierNonArray::IImage1D => quote!{ glsl::syntax::TypeSpecifierNonArray::IImage1D },
    syntax::TypeSpecifierNonArray::ISampler2D => quote!{ glsl::syntax::TypeSpecifierNonArray::ISampler2D },
    syntax::TypeSpecifierNonArray::IImage2D => quote!{ glsl::syntax::TypeSpecifierNonArray::IImage2D },
    syntax::TypeSpecifierNonArray::ISampler3D => quote!{ glsl::syntax::TypeSpecifierNonArray::ISampler3D },
    syntax::TypeSpecifierNonArray::IImage3D => quote!{ glsl::syntax::TypeSpecifierNonArray::IImage3D },
    syntax::TypeSpecifierNonArray::ISamplerCube => quote!{ glsl::syntax::TypeSpecifierNonArray::ISamplerCube },
    syntax::TypeSpecifierNonArray::IImageCube => quote!{ glsl::syntax::TypeSpecifierNonArray::IImageCube },
    syntax::TypeSpecifierNonArray::ISampler2DRect => quote!{ glsl::syntax::TypeSpecifierNonArray::ISampler2DRect },
    syntax::TypeSpecifierNonArray::IImage2DRect => quote!{ glsl::syntax::TypeSpecifierNonArray::IImage2DRect },
    syntax::TypeSpecifierNonArray::ISampler1DArray => quote!{ glsl::syntax::TypeSpecifierNonArray::ISampler1DArray },
    syntax::TypeSpecifierNonArray::IImage1DArray => quote!{ glsl::syntax::TypeSpecifierNonArray::IImage1DArray },
    syntax::TypeSpecifierNonArray::ISampler2DArray => quote!{ glsl::syntax::TypeSpecifierNonArray::ISampler2DArray },
    syntax::TypeSpecifierNonArray::IImage2DArray => quote!{ glsl::syntax::TypeSpecifierNonArray::IImage2DArray },
    syntax::TypeSpecifierNonArray::ISamplerBuffer => quote!{ glsl::syntax::TypeSpecifierNonArray::ISamplerBuffer },
    syntax::TypeSpecifierNonArray::IImageBuffer => quote!{ glsl::syntax::TypeSpecifierNonArray::IImageBuffer },
    syntax::TypeSpecifierNonArray::ISampler2DMS => quote!{ glsl::syntax::TypeSpecifierNonArray::ISampler2DMS },
    syntax::TypeSpecifierNonArray::IImage2DMS => quote!{ glsl::syntax::TypeSpecifierNonArray::IImage2DMS },
    syntax::TypeSpecifierNonArray::ISampler2DMSArray => quote!{ glsl::syntax::TypeSpecifierNonArray::ISampler2DMSArray },
    syntax::TypeSpecifierNonArray::IImage2DMSArray => quote!{ glsl::syntax::TypeSpecifierNonArray::IImage2DMSArray },
    syntax::TypeSpecifierNonArray::ISamplerCubeArray => quote!{ glsl::syntax::TypeSpecifierNonArray::ISamplerCubeArray },
    syntax::TypeSpecifierNonArray::IImageCubeArray => quote!{ glsl::syntax::TypeSpecifierNonArray::IImageCubeArray },
    syntax::TypeSpecifierNonArray::AtomicUInt => quote!{ glsl::syntax::TypeSpecifierNonArray::AtomicUInt },
    syntax::TypeSpecifierNonArray::USampler1D => quote!{ glsl::syntax::TypeSpecifierNonArray::USampler1D },
    syntax::TypeSpecifierNonArray::UImage1D => quote!{ glsl::syntax::TypeSpecifierNonArray::UImage1D },
    syntax::TypeSpecifierNonArray::USampler2D => quote!{ glsl::syntax::TypeSpecifierNonArray::USampler2D },
    syntax::TypeSpecifierNonArray::UImage2D => quote!{ glsl::syntax::TypeSpecifierNonArray::UImage2D },
    syntax::TypeSpecifierNonArray::USampler3D => quote!{ glsl::syntax::TypeSpecifierNonArray::USampler3D },
    syntax::TypeSpecifierNonArray::UImage3D => quote!{ glsl::syntax::TypeSpecifierNonArray::UImage3D },
    syntax::TypeSpecifierNonArray::USamplerCube => quote!{ glsl::syntax::TypeSpecifierNonArray::USamplerCube },
    syntax::TypeSpecifierNonArray::UImageCube => quote!{ glsl::syntax::TypeSpecifierNonArray::UImageCube },
    syntax::TypeSpecifierNonArray::USampler2DRect => quote!{ glsl::syntax::TypeSpecifierNonArray::USampler2DRect },
    syntax::TypeSpecifierNonArray::UImage2DRect => quote!{ glsl::syntax::TypeSpecifierNonArray::UImage2DRect },
    syntax::TypeSpecifierNonArray::USampler1DArray => quote!{ glsl::syntax::TypeSpecifierNonArray::USampler1DArray },
    syntax::TypeSpecifierNonArray::UImage1DArray => quote!{ glsl::syntax::TypeSpecifierNonArray::UImage1DArray },
    syntax::TypeSpecifierNonArray::USampler2DArray => quote!{ glsl::syntax::TypeSpecifierNonArray::USampler2DArray },
    syntax::TypeSpecifierNonArray::UImage2DArray => quote!{ glsl::syntax::TypeSpecifierNonArray::UImage2DArray },
    syntax::TypeSpecifierNonArray::USamplerBuffer => quote!{ glsl::syntax::TypeSpecifierNonArray::USamplerBuffer },
    syntax::TypeSpecifierNonArray::UImageBuffer => quote!{ glsl::syntax::TypeSpecifierNonArray::UImageBuffer },
    syntax::TypeSpecifierNonArray::USampler2DMS => quote!{ glsl::syntax::TypeSpecifierNonArray::USampler2DMS },
    syntax::TypeSpecifierNonArray::UImage2DMS => quote!{ glsl::syntax::TypeSpecifierNonArray::UImage2DMS },
    syntax::TypeSpecifierNonArray::USampler2DMSArray => quote!{ glsl::syntax::TypeSpecifierNonArray::USampler2DMSArray },
    syntax::TypeSpecifierNonArray::UImage2DMSArray => quote!{ glsl::syntax::TypeSpecifierNonArray::UImage2DMSArray },
    syntax::TypeSpecifierNonArray::USamplerCubeArray => quote!{ glsl::syntax::TypeSpecifierNonArray::USamplerCubeArray },
    syntax::TypeSpecifierNonArray::UImageCubeArray => quote!{ glsl::syntax::TypeSpecifierNonArray::UImageCubeArray },
    syntax::TypeSpecifierNonArray::Struct(ref s) => tokenize_struct_non_declaration(s),
    syntax::TypeSpecifierNonArray::TypeName(ref tn) => quote!{#tn}
  }
}

fn tokenize_type_specifier(t: &syntax::TypeSpecifier) -> TokenStream {
  let ty = tokenize_type_specifier_non_array(&t.ty);
  let array_specifier = t.array_specifier.as_ref().map(tokenize_array_spec).quote();

  quote!{
    glsl::syntax::TypeSpecifier {
      ty: #ty,
      array_specifier: #array_specifier
    }
  }
}

fn tokenize_fully_specified_type(t: &syntax::FullySpecifiedType) -> TokenStream {
  let qual = t.qualifier.as_ref().map(tokenize_type_qualifier).quote();
  let ty = tokenize_type_specifier(&t.ty);

  quote!{
    glsl::syntax::FullySpecifiedType {
      qualifier: #qual,
      ty: #ty
    }
  }
}

fn tokenize_struct_non_declaration(s: &syntax::StructSpecifier) -> TokenStream {
  let name = &s.name;
  let fields = s.fields.iter().map(tokenize_struct_field);

  quote!{
    glsl::syntax::StructSpecifier {
      name: String::from(#name),
      fields: vec![#(#fields),*]
    }
  }
}

fn tokenize_struct_field(field: &syntax::StructFieldSpecifier) -> TokenStream {
  let qual = field.qualifier.as_ref().map(tokenize_type_qualifier).quote();
  let ty = tokenize_type_specifier(&field.ty);
  let identifiers = field.identifiers.iter().map(|&(ref a, ref b)| tokenize_arrayed_identifier(a, b));

  quote!{
    glsl::syntax::StructFieldSpecifier {
      qualifier: #qual,
      ty: #ty,
      identifiers: vec![#(#identifiers),*]
    }
  }
}

fn tokenize_array_spec(a: &syntax::ArraySpecifier) -> TokenStream {
  match *a {
    syntax::ArraySpecifier::Unsized => quote!{ glsl::syntax::ArraySpecifier::Unsized },
    syntax::ArraySpecifier::ExplicitlySized(ref e) => {
      let expr = Box::new(tokenize_expr(&e));
      quote!{ glsl::syntax::ArraySpecifier::ExplicitlySized(#expr) }
    }
  }
}

fn tokenize_arrayed_identifier(i: &syntax::Identifier, arr_spec: &Option<syntax::ArraySpecifier>) -> TokenStream {
  let arr_spec = arr_spec.as_ref().map(tokenize_array_spec).quote();
  quote!{ (#i, #arr_spec) }
}

fn tokenize_type_qualifier(q: &syntax::TypeQualifier) -> TokenStream {
  let quals = q.qualifiers.iter().map(tokenize_type_qualifier_spec);

  quote!{
    glsl::syntax::TypeQualifier {
      qualifiers: vec![#(#quals),*]
    }
  }
}

fn tokenize_type_qualifier_spec(q: &syntax::TypeQualifierSpec) -> TokenStream {
  match *q {
    syntax::TypeQualifierSpec::Storage(ref s) => {
      let s = tokenize_storage_qualifier(s);
      quote!{ glsl::syntax::TypeQualifierSpec::Storage(#s) }
    }

    syntax::TypeQualifierSpec::Layout(ref l) => {
      let l = tokenize_layout_qualifier(l);
      quote!{ glsl::syntax::TypeQualifierSpec::Layout(#l) }
    }

    syntax::TypeQualifierSpec::Precision(ref p) => {
      let p = tokenize_precision_qualifier(p);
      quote!{ glsl::syntax::TypeQualifierSpec::Precision(#p) }
    }

    syntax::TypeQualifierSpec::Interpolation(ref i) => {
      let i = tokenize_interpolation_qualifier(i);
      quote!{ glsl::syntax::TypeQualifierSpec::Interpolation(#i) }
    }

    syntax::TypeQualifierSpec::Invariant => quote!{ glsl::syntax::TypeQualifierSpec::Invariant },

    syntax::TypeQualifierSpec::Precise => quote!{ glsl::syntax::TypeQualifierSpec::Precise }
  }
}

fn tokenize_storage_qualifier(q: &syntax::StorageQualifier) -> TokenStream {
  match *q {
    syntax::StorageQualifier::Const => quote!{ glsl::syntax::StorageQualifier::Const },
    syntax::StorageQualifier::InOut => quote!{ glsl::syntax::StorageQualifier::InOut },
    syntax::StorageQualifier::In => quote!{ glsl::syntax::StorageQualifier::In },
    syntax::StorageQualifier::Out => quote!{ glsl::syntax::StorageQualifier::Out },
    syntax::StorageQualifier::Centroid => quote!{ glsl::syntax::StorageQualifier::Centroid },
    syntax::StorageQualifier::Patch => quote!{ glsl::syntax::StorageQualifier::Patch },
    syntax::StorageQualifier::Sample => quote!{ glsl::syntax::StorageQualifier::Sample },
    syntax::StorageQualifier::Uniform => quote!{ glsl::syntax::StorageQualifier::Uniform },
    syntax::StorageQualifier::Buffer => quote!{ glsl::syntax::StorageQualifier::Buffer },
    syntax::StorageQualifier::Shared => quote!{ glsl::syntax::StorageQualifier::Shared },
    syntax::StorageQualifier::Coherent => quote!{ glsl::syntax::StorageQualifier::Coherent },
    syntax::StorageQualifier::Volatile => quote!{ glsl::syntax::StorageQualifier::Volatile },
    syntax::StorageQualifier::Restrict => quote!{ glsl::syntax::StorageQualifier::Restrict },
    syntax::StorageQualifier::ReadOnly => quote!{ glsl::syntax::StorageQualifier::ReadOnly },
    syntax::StorageQualifier::WriteOnly => quote!{ glsl::syntax::StorageQualifier::WriteOnly },
    syntax::StorageQualifier::Subroutine(ref n) => quote!{
      StorageQualifier::Subroutine(vec![#(String::from(#n)),*])
    }
  }
}

fn tokenize_layout_qualifier(l: &syntax::LayoutQualifier) -> TokenStream {
  let ids = l.ids.iter().map(tokenize_layout_qualifier_spec);

  quote!{
    glsl::syntax::LayoutQualifier {
      ids: vec![#(#ids),*]
    }
  }
}

fn tokenize_layout_qualifier_spec(l: &syntax::LayoutQualifierSpec) -> TokenStream {
  match *l {
    syntax::LayoutQualifierSpec::Identifier(ref i, ref e) => {
      let expr = e.as_ref().map(|e| Box::new(tokenize_expr(&e))).quote();
      quote!{ glsl::syntax::LayoutQualifierSpec::Identifier(#i, #expr) }
    }

    syntax::LayoutQualifierSpec::Shared => quote!{ glsl::syntax::LayoutQualifierSpec::Shared }
  }
}

fn tokenize_precision_qualifier(p: &syntax::PrecisionQualifier) -> TokenStream {
  match *p {
    syntax::PrecisionQualifier::High => quote!{ glsl::syntax::PrecisionQualifier::High },
    syntax::PrecisionQualifier::Medium => quote!{ glsl::syntax::PrecisionQualifier::Medium },
    syntax::PrecisionQualifier::Low => quote!{ glsl::syntax::PrecisionQualifier::Low }
  }
}

fn tokenize_interpolation_qualifier(i: &syntax::InterpolationQualifier) -> TokenStream {
  match *i {
    syntax::InterpolationQualifier::Smooth => quote!{ glsl::syntax::InterpolationQualifier::Smooth },
    syntax::InterpolationQualifier::Flat => quote!{ glsl::syntax::InterpolationQualifier::Flat },
    syntax::InterpolationQualifier::NoPerspective => quote!{ glsl::syntax::InterpolationQualifier::NoPerspective }
  }
}

fn tokenize_expr(expr: &syntax::Expr) -> TokenStream {
  match *expr {
    syntax::Expr::Variable(ref i) => quote!{ glsl::syntax::Expr::Variable(#i) },

    syntax::Expr::IntConst(ref x) => quote!{ glsl::syntax::Expr::IntConst(#x) },

    syntax::Expr::UIntConst(ref x) => quote!{ glsl::syntax::Expr::UIntConst(#x) },

    syntax::Expr::BoolConst(ref x) => quote!{ glsl::syntax::Expr::BoolConst(#x) },

    syntax::Expr::FloatConst(ref x) => quote!{ glsl::syntax::Expr::FloatConst(#x) },

    syntax::Expr::DoubleConst(ref x) => quote!{ glsl::syntax::Expr::DoubleConst(#x) },

    syntax::Expr::Unary(ref op, ref e) => {
      let op = tokenize_unary_op(op);
      let e = Box::new(tokenize_expr(e));
      quote!{ glsl::syntax::Expr::Unary(#op, #e) }
    }

    syntax::Expr::Binary(ref op, ref l, ref r) => {
      let op = tokenize_binary_op(op);
      let l = Box::new(tokenize_expr(l));
      let r = Box::new(tokenize_expr(r));
      quote!{ glsl::syntax::Expr::Binary(#op, #l, #r) }
    }

    syntax::Expr::Ternary(ref c, ref s, ref e) => {
      let c = Box::new(tokenize_expr(c));
      let s = Box::new(tokenize_expr(s));
      let e = Box::new(tokenize_expr(e));
      quote!{ glsl::syntax::Expr::Ternary(#c, #s, #e) }
    }

    syntax::Expr::Assignment(ref v, ref op, ref e) => {
      let v = Box::new(tokenize_expr(v));
      let op = tokenize_assignment_op(op);
      let e = Box::new(tokenize_expr(e));
      quote!{ glsl::syntax::Expr::Assignment(#v, #op, #e) }
    }

    syntax::Expr::Bracket(ref e, ref a) => {
      let e = Box::new(tokenize_expr(e));
      let a = tokenize_array_spec(a);
      quote!{ glsl::syntax::Expr::Bracket(#e, #a) }
    }

    syntax::Expr::FunCall(ref fun, ref args) => {
      let fun = tokenize_function_identifier(fun);
      let args = args.iter().map(tokenize_expr);
      quote!{ glsl::syntax::Expr::FunCall(#fun, vec![#(#args),*]) }
    }

    syntax::Expr::Dot(ref e, ref i) => {
      let e = Box::new(tokenize_expr(e));
      quote!{ glsl::syntax::Expr::Dot(#e, #i) }
    }

    syntax::Expr::PostInc(ref e) => {
      let e = Box::new(tokenize_expr(e));
      quote!{ glsl::syntax::Expr::PostInc(#e) }
    }

    syntax::Expr::PostDec(ref e) => {
      let e = Box::new(tokenize_expr(e));
      quote!{ glsl::syntax::Expr::PostDec(#e) }
    }

    syntax::Expr::Comma(ref a, ref b) => {
      let a = Box::new(tokenize_expr(a));
      let b = Box::new(tokenize_expr(b));
      quote!{ glsl::syntax::Expr::Comma(#a, #b) }
    }
  }
}

fn tokenize_unary_op(op: &syntax::UnaryOp) -> TokenStream {
  match *op {
    syntax::UnaryOp::Inc => quote!{ glsl::syntax::UnaryOp::Inc },
    syntax::UnaryOp::Dec => quote!{ glsl::syntax::UnaryOp::Dec },
    syntax::UnaryOp::Add => quote!{ glsl::syntax::UnaryOp::Add },
    syntax::UnaryOp::Minus => quote!{ glsl::syntax::UnaryOp::Minus },
    syntax::UnaryOp::Not => quote!{ glsl::syntax::UnaryOp::Not },
    syntax::UnaryOp::Complement => quote!{ glsl::syntax::UnaryOp::Complement }
  }
}

fn tokenize_binary_op(op: &syntax::BinaryOp) -> TokenStream {
  match *op {
    syntax::BinaryOp::Or => quote!{ glsl::syntax::BinaryOp::Or },
    syntax::BinaryOp::Xor => quote!{ glsl::syntax::BinaryOp::Xor },
    syntax::BinaryOp::And => quote!{ glsl::syntax::BinaryOp::And },
    syntax::BinaryOp::BitOr => quote!{ glsl::syntax::BinaryOp::BitOr },
    syntax::BinaryOp::BitXor => quote!{ glsl::syntax::BinaryOp::BitXor },
    syntax::BinaryOp::BitAnd => quote!{ glsl::syntax::BinaryOp::BitAnd },
    syntax::BinaryOp::Equal => quote!{ glsl::syntax::BinaryOp::Equal },
    syntax::BinaryOp::NonEqual => quote!{ glsl::syntax::BinaryOp::NonEqual },
    syntax::BinaryOp::LT => quote!{ glsl::syntax::BinaryOp::LT },
    syntax::BinaryOp::GT => quote!{ glsl::syntax::BinaryOp::GT },
    syntax::BinaryOp::LTE => quote!{ glsl::syntax::BinaryOp::LTE },
    syntax::BinaryOp::GTE => quote!{ glsl::syntax::BinaryOp::GTE },
    syntax::BinaryOp::LShift => quote!{ glsl::syntax::BinaryOp::LShift },
    syntax::BinaryOp::RShift => quote!{ glsl::syntax::BinaryOp::RShift },
    syntax::BinaryOp::Add => quote!{ glsl::syntax::BinaryOp::Add },
    syntax::BinaryOp::Sub => quote!{ glsl::syntax::BinaryOp::Sub },
    syntax::BinaryOp::Mult => quote!{ glsl::syntax::BinaryOp::Mult },
    syntax::BinaryOp::Div => quote!{ glsl::syntax::BinaryOp::Div },
    syntax::BinaryOp::Mod => quote!{ glsl::syntax::BinaryOp::Mod }
  }
}

fn tokenize_assignment_op(op: &syntax::AssignmentOp) -> TokenStream {
  match *op {
    syntax::AssignmentOp::Equal => quote!{ glsl::syntax::AssignmentOp::Equal },
    syntax::AssignmentOp::Mult => quote!{ glsl::syntax::AssignmentOp::Mult },
    syntax::AssignmentOp::Div => quote!{ glsl::syntax::AssignmentOp::Div },
    syntax::AssignmentOp::Mod => quote!{ glsl::syntax::AssignmentOp::Mod },
    syntax::AssignmentOp::Add => quote!{ glsl::syntax::AssignmentOp::Add },
    syntax::AssignmentOp::Sub => quote!{ glsl::syntax::AssignmentOp::Sub },
    syntax::AssignmentOp::LShift => quote!{ glsl::syntax::AssignmentOp::LShift },
    syntax::AssignmentOp::RShift => quote!{ glsl::syntax::AssignmentOp::RShift },
    syntax::AssignmentOp::And => quote!{ glsl::syntax::AssignmentOp::And },
    syntax::AssignmentOp::Xor => quote!{ glsl::syntax::AssignmentOp::Xor },
    syntax::AssignmentOp::Or => quote!{ AssignmentOp::Or }
  }
}

fn tokenize_function_identifier(i: &syntax::FunIdentifier) -> TokenStream {
  match *i {
    syntax::FunIdentifier::Identifier(ref n) =>
      quote!{ glsl::syntax::FunIdentifier::Identifier(String::from(#n)) },

    syntax::FunIdentifier::Expr(ref e) => {
      let e = tokenize_expr(e);
      quote!{ glsl::syntax::FunIdentifier::Expr(#e) }
    }
  }
}

fn tokenize_declaration(d: &syntax::Declaration) -> TokenStream {
  match *d {
    syntax::Declaration::FunctionPrototype(ref proto) => {
      let p = tokenize_function_prototype(proto);
      quote!{ glsl::syntax::Declaration::FunctionPrototype(#p) }
    }

    syntax::Declaration::InitDeclaratorList(ref list) => {
      let l = tokenize_init_declarator_list(list);
      quote!{ glsl::syntax::Declaration::InitDeclaratorList(#l) }
    }

    syntax::Declaration::Precision(ref qual, ref ty) => {
      let qual = tokenize_precision_qualifier(qual);
      let ty = tokenize_type_specifier(ty);
      quote!{ glsl::syntax::Declaration::Precision(#qual, #ty) }
    }

    syntax::Declaration::Block(ref block) => {
      let block = tokenize_block(block);
      quote!{ glsl::syntax::Declaration::Block(#block) }
    }

    syntax::Declaration::Global(ref qual, ref identifiers) => {
      let qual = tokenize_type_qualifier(qual);
      quote!{ glsl::syntax::Declaration::Global(#qual, vec![#(#identifiers),*]) }
    }
  }
}

fn tokenize_function_prototype(fp: &syntax::FunctionPrototype) -> TokenStream {
  let ty = tokenize_fully_specified_type(&fp.ty);
  let name = &fp.name;
  let params = fp.parameters.iter().map(tokenize_function_parameter_declaration);

  quote!{
    glsl::syntax::FunctionPrototype {
      ty: #ty,
      name: String::from(#name),
      parameters: vec![#(#params),*]
    }
  }
}

fn tokenize_function_parameter_declaration(p: &syntax::FunctionParameterDeclaration) -> TokenStream {
  match *p {
    syntax::FunctionParameterDeclaration::Named(ref qual, ref fpd) => {
      let qual = qual.as_ref().map(tokenize_type_qualifier).quote();
      let fpd = tokenize_function_parameter_declarator(fpd);
      quote!{ glsl::syntax::FunctionParameterDeclaration::Named(#qual, #fpd) }
    }

    syntax::FunctionParameterDeclaration::Unnamed(ref qual, ref ty) => {
      let qual = qual.as_ref().map(tokenize_type_qualifier).quote();
      let ty = tokenize_type_specifier(ty);
      quote!{ glsl::syntax::FunctionParameterDeclaration::Unnamed(#qual, #ty) }
    }
  }
}

fn tokenize_function_parameter_declarator(p: &syntax::FunctionParameterDeclarator) -> TokenStream {
  let ty = tokenize_type_specifier(&p.ty);
  let name = &p.name;
  let a = p.array_spec.as_ref().map(tokenize_array_spec).quote();

  quote!{
    glsl::syntax::FunctionParameterDeclarator {
      ty: #ty,
      name: String::from(#name),
      array_spec: #a
    }
  }
}

fn tokenize_init_declarator_list(i: &syntax::InitDeclaratorList) -> TokenStream {
  let head = tokenize_single_declaration(&i.head);
  let tail = i.tail.iter().map(tokenize_single_declaration_no_type);

  quote!{
    glsl::syntax::InitDeclaratorList {
      head: #head,
      tail: vec![#(#tail),*]
    }
  }
}

fn tokenize_single_declaration(d: &syntax::SingleDeclaration) -> TokenStream {
  let ty = tokenize_fully_specified_type(&d.ty);
  let name = &d.name;
  let array_specifier = d.array_specifier.as_ref().map(tokenize_array_spec).quote();
  let initializer = d.initializer.as_ref().map(tokenize_initializer).quote();

  quote!{
    glsl::syntax::SingleDeclaration {
      ty: #ty,
      name: String::from(#name),
      array_specifier: #array_specifier,
      initializer: #initializer
    }
  }
}

fn tokenize_single_declaration_no_type(d: &syntax::SingleDeclarationNoType) -> TokenStream {
  let name = &d.name;
  let array_specifier = d.array_specifier.as_ref().map(tokenize_array_spec).quote();
  let initializer = d.initializer.as_ref().map(tokenize_initializer).quote();

  quote!{
    glsl::syntax::SingleDeclarationNoType {
      name: String::from(#name),
      array_specifier: #array_specifier,
      initializer: #initializer
    }
  }
}

fn tokenize_initializer(i: &syntax::Initializer) -> TokenStream {
  match *i {
    syntax::Initializer::Simple(ref e) => {
      let e = tokenize_expr(e);
      quote!{ glsl::syntax::Initializer::Simple(#e) }
    }

    syntax::Initializer::List(ref list) => {
      let l = list.iter().map(tokenize_initializer);
      quote!{ glsl::syntax::Initializer::List(vec![#(#l),*]) }
    }
  }
}

fn tokenize_block(b: &syntax::Block) -> TokenStream {
  let qual = tokenize_type_qualifier(&b.qualifier);
  let name = &b.name;
  let fields = b.fields.iter().map(tokenize_struct_field);
  let identifier = b.identifier.as_ref().map(|&(ref a, ref b)| tokenize_arrayed_identifier(a, b)).quote();

  quote!{
    glsl::syntax::Block {
      qualifier: #qual,
      name: String::from(#name),
      fields: vec![#(#fields),*],
      identifier: #identifier
    }
  }
}

fn tokenize_function_definition(fd: &syntax::FunctionDefinition) -> TokenStream {
  let p = tokenize_function_prototype(&fd.prototype);
  let s = tokenize_compound_statement(&fd.statement);

  quote!{
    glsl::syntax::FunctionDefinition {
      prototype: #p,
      statement: #s
    }
  }
}

fn tokenize_compound_statement(cst: &syntax::CompoundStatement) -> TokenStream {
  let s = cst.statement_list.iter().map(tokenize_statement);

  quote!{
    glsl::syntax::CompoundStatement {
      statement_list: vec![#(#s),*]
    }
  }
}

fn tokenize_statement(st: &syntax::Statement) -> TokenStream {
  match *st {
    syntax::Statement::Compound(ref cst) => {
      let s = Box::new(tokenize_compound_statement(cst));
      quote!{ glsl::syntax::Statement::Compound(#s) }
    }

    syntax::Statement::Simple(ref sst) => {
      let s = Box::new(tokenize_simple_statement(sst));
      quote!{ glsl::syntax::Statement::Simple(#s) }
    }
  }
}

fn tokenize_simple_statement(sst: &syntax::SimpleStatement) -> TokenStream {
  match *sst {
    syntax::SimpleStatement::Declaration(ref d) => {
      let d = tokenize_declaration(d);
      quote!{ glsl::syntax::SimpleStatement::Declaration(#d) }
    }

    syntax::SimpleStatement::Expression(ref e) => {
      let e = tokenize_expression_statement(e);
      quote!{ glsl::syntax::SimpleStatement::Expression(#e) }
    }

    syntax::SimpleStatement::Selection(ref s) => {
      let s = tokenize_selection_statement(s);
      quote!{ glsl::syntax::SimpleStatement::Selection(#s) }
    }

    syntax::SimpleStatement::Switch(ref s) => {
      let s = tokenize_switch_statement(s);
      quote!{ glsl::syntax::SimpleStatement::Switch(#s) }
    }

    syntax::SimpleStatement::CaseLabel(ref cl) => {
      let cl = tokenize_case_label(cl);
      quote!{ glsl::syntax::SimpleStatement::CaseLabel(#cl) }
    }

    syntax::SimpleStatement::Iteration(ref i) => {
      let i = tokenize_iteration_statement(i);
      quote!{ glsl::syntax::SimpleStatement::Iteration(#i) }
    }

    syntax::SimpleStatement::Jump(ref j) => {
      let j = tokenize_jump_statement(j);
      quote!{ glsl::syntax::SimpleStatement::Jump(#j) }
    }
  }
}

fn tokenize_expression_statement(est: &syntax::ExprStatement) -> TokenStream {
  let e = est.as_ref().map(|e| tokenize_expr(&e)).quote();
  quote!{#e}
}

fn tokenize_selection_statement(sst: &syntax::SelectionStatement) -> TokenStream {
  let cond = Box::new(tokenize_expr(&sst.cond));
  let rest = tokenize_selection_rest_statement(&sst.rest);

  quote!{
    glsl::syntax::SelectionStatement {
      cond: #cond,
      rest: #rest
    }
  }
}

fn tokenize_selection_rest_statement(sst: &syntax::SelectionRestStatement) -> TokenStream {
  match *sst {
    syntax::SelectionRestStatement::Statement(ref if_st) => {
      let e = Box::new(tokenize_statement(if_st));
      quote!{ glsl::syntax::SelectionRestStatement::Statement(#e) }
    }

    syntax::SelectionRestStatement::Else(ref if_st, ref else_st) => {
      let if_st = Box::new(tokenize_statement(if_st));
      let else_st = Box::new(tokenize_statement(else_st));
      quote!{ glsl::syntax::SelectionRestStatement::Else(#if_st, #else_st) }
    }
  }
}

fn tokenize_switch_statement(sst: &syntax::SwitchStatement) -> TokenStream {
  let head = Box::new(tokenize_expr(&sst.head));
  let body = sst.body.iter().map(tokenize_statement);

  quote!{
    glsl::syntax::SwitchStatement {
      head: #head,
      body: vec![#(#body),*]
    }
  }
}

fn tokenize_case_label(cl: &syntax::CaseLabel) -> TokenStream {
  match *cl {
    syntax::CaseLabel::Case(ref e) => {
      let e = Box::new(tokenize_expr(e));
      quote!{ glsl::syntax::CaseLabel::Case(#e) }
    }

    syntax::CaseLabel::Def => quote!{ glsl::syntax::CaseLabel::Def }
  }
}

fn tokenize_iteration_statement(ist: &syntax::IterationStatement) -> TokenStream {
  match *ist {
    syntax::IterationStatement::While(ref cond, ref body) => {
      let cond = tokenize_condition(cond);
      let body = Box::new(tokenize_statement(body));
      quote!{ glsl::syntax::IterationStatement::While(#cond, #body) }
    }

    syntax::IterationStatement::DoWhile(ref body, ref cond) => {
      let body = Box::new(tokenize_statement(body));
      let cond = Box::new(tokenize_expr(cond));
      quote!{ glsl::syntax::IterationStatement::DoWhile(#body, #cond) }
    }

    syntax::IterationStatement::For(ref init, ref rest, ref body) => {
      let init = tokenize_for_init_statement(init);
      let rest = tokenize_for_rest_statement(rest);
      let body = Box::new(tokenize_statement(body));
      quote!{ glsl::syntax::IterationStatement::For(#init, #rest, #body) }
    }
  }
}

fn tokenize_condition(c: &syntax::Condition) -> TokenStream {
  match *c {
    syntax::Condition::Expr(ref e) => {
      let e = Box::new(tokenize_expr(e));
      quote!{ glsl::syntax::Condition::Expr(#e) }
    }

    syntax::Condition::Assignment(ref ty, ref name, ref initializer) => {
      let ty = tokenize_fully_specified_type(ty);
      let initializer = tokenize_initializer(initializer);
      quote!{ glsl::syntax::Condition::Assignment(#ty, String::from(#name), #initializer) }
    }
  }
}

fn tokenize_for_init_statement(i: &syntax::ForInitStatement) -> TokenStream {
  match *i {
    syntax::ForInitStatement::Expression(ref expr) => {
      let e = expr.as_ref().map(|e| tokenize_expr(&e)).quote();
      quote!{ glsl::syntax::ForInitStatement::Expression(#e) }
    }

    syntax::ForInitStatement::Declaration(ref d) => {
      let d = Box::new(tokenize_declaration(d));
      quote!{ glsl::syntax::ForInitStatement::Declaration(#d) }
    }
  }
}

fn tokenize_for_rest_statement(r: &syntax::ForRestStatement) -> TokenStream {
  let cond = r.condition.as_ref().map(tokenize_condition).quote();
  let post = r.post_expr.as_ref().map(|e| Box::new(tokenize_expr(&e))).quote();

  quote!{
    glsl::syntax::ForRestStatement {
      condition: #cond,
      post: #post
    }
  }
}

fn tokenize_jump_statement(j: &syntax::JumpStatement) -> TokenStream {
  match *j {
    syntax::JumpStatement::Continue => quote!{ JumpStatement::Continue },
    syntax::JumpStatement::Break => quote!{ JumpStatement::Break },
    syntax::JumpStatement::Discard => quote!{ JumpStatement::Discard },
    syntax::JumpStatement::Return(ref e) => {
      let e = Box::new(tokenize_expr(e));
      quote!{ JumpStatement::Return(#e) }
    }
  }
}

fn tokenize_preprocessor(pp: &syntax::Preprocessor) -> TokenStream {
  match *pp {
    syntax::Preprocessor::Version(ref pv) => {
      let pv = tokenize_preprocessor_version(pv);
      quote!{ glsl::syntax::Preprocessor::Version(#pv) }
    }

    syntax::Preprocessor::Extension(ref pe) => {
      let pe = tokenize_preprocessor_extension(pe);
      quote!{ glsl::syntax::Preprocessor::Extension(#pe) }
    }
  }
}

fn tokenize_preprocessor_version(pv: &syntax::PreprocessorVersion) -> TokenStream {
  let version = pv.version;
  let profile = pv.profile.as_ref().map(tokenize_preprocessor_version_profile).quote();

  quote!{
    glsl::syntax::PreprocessorVersion {
      version: #version,
      profile: #profile
    }
  }
}

fn tokenize_preprocessor_version_profile(profile: &syntax::PreprocessorVersionProfile) -> TokenStream {
  match *profile {
    syntax::PreprocessorVersionProfile::Core => quote!{ glsl::syntax::PreprocessorVersionProfile::Core },
    syntax::PreprocessorVersionProfile::Compatibility => quote!{ glsl::syntax::PreprocessorVersionProfile::Compatibility },
    syntax::PreprocessorVersionProfile::ES => quote!{ glsl::syntax::PreprocessorVersionProfile::ES }
  }
}

fn tokenize_preprocessor_extension(pe: &syntax::PreprocessorExtension) -> TokenStream {
  let name = tokenize_preprocessor_extension_name(&pe.name);
  let behavior = pe.behavior.as_ref().map(tokenize_preprocessor_extension_behavior).quote();

  quote!{
    glsl::syntax::PreprocessorExtension {
      name: String::from(#name),
      behavior: #behavior
    }
  }
}

fn tokenize_preprocessor_extension_name(name: &syntax::PreprocessorExtensionName) -> TokenStream {
  match *name {
    syntax::PreprocessorExtensionName::All => quote!{ glsl::syntax::PreprocessorExtensionName::All },
    syntax::PreprocessorExtensionName::Specific(ref n) =>
      quote!{ glsl::syntax::PreprocessorExtensionName::Specific(String::from(#n)) }
  }
}

fn tokenize_preprocessor_extension_behavior(behavior: &syntax::PreprocessorExtensionBehavior) -> TokenStream {
  match *behavior {
    syntax::PreprocessorExtensionBehavior::Require => quote!{ glsl::syntax::PreprocessorExtensionBehavior::Require },
    syntax::PreprocessorExtensionBehavior::Enable => quote!{ glsl::syntax::PreprocessorExtensionBehavior::Enable },
    syntax::PreprocessorExtensionBehavior::Warn => quote!{ glsl::syntax::PreprocessorExtensionBehavior::Warn },
    syntax::PreprocessorExtensionBehavior::Disable => quote!{ glsl::syntax::PreprocessorExtensionBehavior::Disable }
  }
}

fn tokenize_external_declaration(ed: &syntax::ExternalDeclaration) -> TokenStream {
  match *ed {
    syntax::ExternalDeclaration::Preprocessor(ref pp) => {
      let pp = tokenize_preprocessor(pp);
      quote!{ glsl::syntax::ExternalDeclaration::Preprocessor(#pp) }
    }

    syntax::ExternalDeclaration::FunctionDefinition(ref fd) => {
      let fd = tokenize_function_definition(fd);
      quote!{ glsl::syntax::ExternalDeclaration::FunctionDefinition(#fd) }
    }

    syntax::ExternalDeclaration::Declaration(ref d) => {
      let d = tokenize_declaration(d);
      quote!{ glsl::syntax::ExternalDeclaration::Declaration(#d) }
    }
  }
}

fn tokenize_translation_unit(tu: &syntax::TranslationUnit) -> TokenStream {
  let tu = tu.iter().map(tokenize_external_declaration);
  quote!{vec![#(#tu),*]}
}
