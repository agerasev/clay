use std::collections::HashSet;
use ocl::{self, prm, builders::KernelBuilder};
use nalgebra::{Vector3, Matrix3};
use crate::{Push, View};

pub struct ProjView {
    pub pos: Vector3<f64>,
    pub ori: Matrix3<f64>,
}

impl View for ProjView {
	fn source(_: &mut HashSet<u64>) -> String {
		"#include <clay_core/view/proj_view.h>\n".to_string()
	}
}

impl Push for ProjView {
    fn args_def(kb: &mut KernelBuilder) {
        kb
        .arg(prm::Float3::zero())
        .arg(prm::Float16::zero());
    }
    fn args_set(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let mapf = self.ori.map(|x| x as f32);
        let mut map16 = [0f32; 16];
        map16[0..3].copy_from_slice(&mapf.as_slice()[0..3]);
        map16[4..7].copy_from_slice(&mapf.as_slice()[3..6]);
        map16[8..11].copy_from_slice(&mapf.as_slice()[6..9]);

        let posf = self.pos.map(|x| x as f32);
        let mut pos3 = [0f32; 3];
        pos3.copy_from_slice(posf.as_slice());

        k.set_arg(i + 0, &prm::Float3::from(pos3))?;
        k.set_arg(i + 1, &prm::Float16::from(map16))?;

        Ok(())
    }
    fn args_count() -> usize {
        2
    }
}
