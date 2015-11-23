use basic::*;
pub use objects::aabb::*;

/// Trait for finding ray intersections.
/// Instances **must** satisfy the law:
///
/// ```text
/// s.is_hit_by(ray, tmax) == s.intersect(ray, tmax).is_some()
/// ```
pub trait Surface {
    /// Returns information about the intersection of the object and the ray, if one exists.
    /// If the distance is greater that `t_max`, it returns `None`.
    fn intersect(&self, ray: Ray, t_max: f64) -> Option<DelayedIntersection>;

    /// Checks whether the ray intersects the object, computes no additional information.
    /// If the distance is greater than `t_max`, it returns `false`.
    fn is_hit_by(&self, ray: Ray, t_max: f64) -> bool;

    /// Returns a finite (!) axis-aligned bounding box if one exists.
    fn bounding_box(&self) -> Option<Aabb>;
}
