use std::rc::Rc;
use crate::hittable::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {objects: Vec::new()}
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>){
        self.objects.push(object);
    } 
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut ret: Option<HitRecord> = None;
        for object in &self.objects {
            if let Some(hit_record) = object.hit(r, t_min, closest_so_far) {
                if hit_record.t < closest_so_far {
                    closest_so_far = hit_record.t;
                    ret = Some(hit_record)
                }
            }
        };
        ret
    }
}