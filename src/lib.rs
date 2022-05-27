#![feature(adt_const_params, generic_associated_types)]
#![allow(incomplete_features)]

mod basic;
mod gat;
mod lifetime_parameterized;

struct Field<const NAME: &'static str>;

struct Dog {
    height: f32,
    age: u32,
}
