use lin_alg::*;

/// Computes the specular coefficient.
pub fn compute_specular(ray_origin_dir: UnitVec3, light_dir: UnitVec3, normal: UnitVec3, shininess: f64) -> f64 {
    let halfway = (light_dir + ray_origin_dir).normalize();
    f64::max(0.0, halfway * normal).powf(shininess)
}

/// Computes the reflection of i along n.
pub fn reflect(i: UnitVec3, n: UnitVec3) -> UnitVec3 {
    Vec3::assert_unit_vector(i - 2. * (i * n) * n)
}

/// Computes the Fresnel reflection of i on an object surface with normal n and index of refraction (ior) r.
/// Precisely speaking, r = ior of material being entered / ior of material being exited.
pub fn fresnel(i: UnitVec3, n: UnitVec3, r: f64) -> f64 {
    let c = -i * n; // = cos(angle of incidence)
    let g = (r * r + c * c - 1.).sqrt(); // = r * cos(angle of refraction)
    let gpc = g + c;
    let gmc = g - c;
    let f1 = gmc / gpc; // = sqrt(reflectance for orthogonal polarization)
    let f2 = (c * gpc - 1.) / (c * gmc + 1.); // f1 * f2 = sqrt(reflectance for parallel polarization)
    f1 * f1 * (f2 * f2 + 1.) / 2. // = average of reflectance for orthogonal and parallel polarization
}

/// Calculates the refraction of i on an object surface with normal n and refraction index quotient r.
/// Precisely speaking, r = ior of material being entered / ior of material being exited.
pub fn refract(i: UnitVec3, n: UnitVec3, r: f64) -> Option<UnitVec3> {
    let r = 1. / r;
    let w = -r * i * n;
    let k = 1. + (w + r) * (w - r);
    if k < 0. {
        None
    } else {
        Some(Vec3::assert_unit_vector(r * i + (w - k.sqrt()) * n))
    }
}
