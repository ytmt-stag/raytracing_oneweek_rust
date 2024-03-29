use super::vec::Vec3;

/// ### Ray Function
/// ```
/// p(t) = A + tB
/// ```
/// A = ray origin, B = ray direction, t = ray parameter
#[derive(Clone, Copy, Debug)]
pub struct Ray
{
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray
{
    pub fn new(origin: Vec3, direction: Vec3) -> Ray
    {
        Ray { 
            origin: origin,
            direction: direction,
        }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3
    {
        self.origin + t * self.direction
    }
}