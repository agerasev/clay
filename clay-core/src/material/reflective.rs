use crate::{pack::*, class::*, material::*};


#[derive(Clone, Debug, Default)]
pub struct Reflective {}

impl Material for Reflective {}

impl Instance<MaterialClass> for Reflective {
    fn source() -> String {
    	"#include <clay_core/material/reflective.h>".to_string()
    }
    fn inst_name() -> String {
        "reflective".to_string()
    }
}

impl Pack for Reflective {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}
