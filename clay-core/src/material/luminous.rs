use std::collections::HashSet;
use crate::{pack::*, class::*, material::*};


#[derive(Clone, Debug, Default)]
pub struct Luminous {}

impl Material for Luminous {}

impl Instance<MaterialClass> for Luminous {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay_core/material/luminous.h>".to_string()
    }
    fn inst_name() -> String {
        "luminous".to_string()
    }
}

impl Pack for Luminous {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}