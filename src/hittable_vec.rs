use crate::hittable::{Hit, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct HittableVec {
    inner: Vec<Box<dyn Hittable>>,
}

impl Deref for HittableVec {
    type Target = Vec<Box<dyn Hittable>>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl DerefMut for HittableVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
    }
}

impl Hittable for HittableVec {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(Hit, &Material)> {
        let mut t_closest = t_max;
        let mut hit_closest = None;
        for h in self.inner.iter() {
            if let Some((hit, mat)) = h.hit(r, t_min, t_closest) {
                t_closest = hit.t();
                hit_closest = Some((hit, mat));
            }
        }
        hit_closest
    }
}

impl HittableVec {
    pub fn new() -> HittableVec {
        HittableVec { inner: Vec::new() }
    }
}
