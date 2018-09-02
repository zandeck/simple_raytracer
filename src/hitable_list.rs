use hitable::*;
use cgmath;
use ray::Ray;

type V = cgmath::Vector3<f64>;

pub struct HitableList<T: Hitable> {
    pub objects: Vec<T>
}

impl<T> Hitable for HitableList<T> where T: Hitable {
    fn hit( &self, r: &Ray, tmin: f64, tmax: f64 ) -> Option<HitRecord> {
        let mut closest_so_far = tmax;
        let mut temp_hit_record = None;

        for o in self.objects.iter() {
            match o.hit(r, tmin, closest_so_far) {
                Some( hr ) => {
                    closest_so_far = hr.t;
                    temp_hit_record = Some(hr);
                }
                None => ()
            }
        }
        temp_hit_record
    }
}