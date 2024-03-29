extern crate math;

use math::vec::*;
use math::ray::*;
use super::ppm_util::*;

/// create simple sphere image
pub fn ch04_add_sphere(nx: i32, ny: i32)
{
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    
    ppm_print_header(nx, ny);
    for y in (0..ny).rev()
    {
        for x in 0..nx
        {
            let u = (x as f64) / (nx as f64);
            let v = (y as f64) / (ny as f64);
            let direction = lower_left_corner + u * horizontal + v * vertical;
            let r = Ray::new(origin, direction);
            let col = color(r);

            ppm_print_rgb(col.r(), col.g(), col.b());
        }
    }
}

/// create color from Ray
fn color(r: Ray) -> Vec3
{
    // if ray hits the sphere, paint to screen
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r)
    {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    // if ray doesn't hit the sphere, paint background
    let unit_direction = r.direction.make_unit_vec();
    let t = 0.5 * (unit_direction.y + 1.0);
    let a = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0);
    let tb = t * Vec3::new(0.5, 0.7, 1.0);
    
    a + tb
}

/// judge if hit sphere
fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> bool
{
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * dot(r.direction, oc);
    let c = dot(oc, oc) - (radius * radius);

    // b^2 - 4ac > 0
    let discriminant = b*b - 4.0 * a * c;    
    discriminant > 0.0
}