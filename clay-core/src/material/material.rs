use vecmat::vec::*;
use crate::{
    Pack,
    class::*,
    material::Colored,
};


/// Material of an object surface.
/// It specifies the way how does ray bounce off the surface.
/// It defines the color, specularity, opacity, diffusion,
/// radiance and other properties of the object surface. 
pub trait Material: Pack + Instance<MaterialClass> + Sized + 'static {
    /// Applies color filter to the material
    fn color_with(self, color: Vec3<f64>) -> Colored<Self> {
        Colored::new(self, color)
    }
}

pub enum MaterialClass {}
impl Class for MaterialClass {
    fn name() -> String {
        "material".to_string()
    }
    fn methods() -> Vec<String> {
        vec!["emit".to_string()]
    }
}
