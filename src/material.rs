use color::*;

/// Represents a material of an object.
#[derive(Debug, Copy, Clone)]
pub struct Material {
    /// Color of the material.
    pub color: Color,
    /// Ambient reflection constant.
    pub ambient: f64,
    /// Diffuse reflection constant.
    pub diffuse: f64,
    /// Specular reflection constant.
    pub specular: f64,
    /// Shininess/specular exponent.
    /// When it is large, the specular highlight is small.
    /// It is larger for smoother and mirror-like surfaces.
    pub shininess: f64,
    /// Mirror reflectance. 0 means no reflection, 1 means perfect mirror.
    pub reflectance: f64,
    /// Refractivity. 0 means no refraction, 1 means only recfraction.
    pub refractivity: f64,
    /// Refraction index. 1 is vacuum.
    pub refraction_index: f64,
}

pub fn vacuum() -> Material {
    Material {
        color: black(),
        ambient: 0.,
        diffuse: 0.,
        specular: 0.,
        shininess: 1.,
        reflectance: 0.,
        refractivity: 1.,
        refraction_index: 1.
    }
}

pub fn reflective_material(reflectance: f64, color: Color) -> Material {
    Material { color: color, reflectance: reflectance, diffuse: 0.9 - reflectance, specular: 0.1, shininess: 50., refractivity: 0., .. vacuum() }
}

pub fn glass() -> Material {
    Material { refraction_index: 1.5, refractivity: 0.9, specular: 0.1, shininess: 200., color: white(), .. vacuum() }
}

pub fn neutral_material() -> Material {
    Material {
        color: black(),
        ambient: 0.2,
        diffuse: 0.6,
        specular: 0.2,
        shininess: 10.,
        reflectance: 0.,
        refractivity: 0.,
        refraction_index: 1.
    }
}

pub fn color_material(c: Color) -> Material {
    Material { color: c, .. neutral_material()}
}
