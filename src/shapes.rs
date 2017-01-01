use ray::Ray;
use point::Point;

pub enum Intersection {
    Hit(f64),
    Miss
}

pub trait Intersectable {
    fn intersects(&self, ray: &Ray) -> Intersection;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sphere {
    pub origin: Point,
    pub radius: f64
}

impl Intersectable for Sphere {
    fn intersects(&self, ray: &Ray) -> Intersection {
        let l = self.origin - ray.origin;
        let tca = l.dot(ray.direction);
        let radius2 = self.radius * self.radius;

        if tca < 0.0 {
            return Intersection::Miss;
        }

        let d2 = l.dot(l) - tca * tca;
        if d2 > radius2 {
            return Intersection::Miss;
        }

        let thc = (radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;

        let final_t0;
        let final_t1;

        if t0 > t1 {
            final_t0 = t0;
            final_t1 = t1;
        } else {
            final_t0 = t1;
            final_t1 = t0;
        }

        if final_t0 < 0.0 { 
            if final_t1 < 0.0 {
                return Intersection::Miss; // both t0 and t1 are negative 
            }
        } 

        if final_t0 > final_t1 {
            return Intersection::Hit(final_t0)
        }

        return Intersection::Hit(final_t1)
    }
}
