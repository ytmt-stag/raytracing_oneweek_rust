extern crate math;

use math::vec::*;
use math::ray::Ray;

use super::hitable_trait::*;

pub struct Sphere
{
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere
{
    pub fn new(center: Vec3, radius: f64) -> Sphere
    {
        Sphere { 
            center: center,
            radius: radius,
        }
    }
}

impl Hitable for Sphere
{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool
    {
        let oc = r.origin - self.center;
        let a = dot(r.direction, r.direction);
        let b = dot(oc, r.direction);
        let c = dot(oc, oc) - (self.radius * self.radius);

        let discriminant = b*b - a*c;
        let discriminant_sq = discriminant.sqrt();
        if discriminant > 0.0
        {
            let temp = (-b - discriminant_sq) / a;
            if t_min < temp && temp < t_max
            {
                update_record_point(rec, &r, self.center, self.radius, temp);
                return true;
            }

            let temp = (-b + discriminant_sq) / a;
            if t_min < temp && temp < t_max
            {
                update_record_point(rec, &r, self.center, self.radius, temp);
                return true;
            }
        }

        false
    }
}

fn update_record_point(rec: &mut HitRecord, ray: &Ray, center: Vec3, radius: f64, discriminant: f64)
{
    rec.t = discriminant;
    rec.p = ray.point_at_parameter(discriminant);
    rec.normal = (rec.p - center) / radius;
}