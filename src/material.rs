use crate::{Mre, Uniform};

#[derive(Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Material {
    albedo: Uniform,
    metallic_roughness_emission: Mre,
    normal_texture: Option<u8>,
    specular: f32,
}

impl ArchivedMaterial {
    #[cfg(feature = "safe")]
    const fn normal_texture(&self) -> Option<u8> {
        match self.normal_texture {
            rkyv::option::ArchivedOption::None => None,
            rkyv::option::ArchivedOption::Some(x) => Some(x),
        }
    }
    #[cfg(feature = "safe")]
    pub(crate) const fn max_texture_idx(&self) -> Option<u8> {
        let t = self.albedo.texture();
        let m = self.metallic_roughness_emission.texture();
        let n = self.normal_texture();
        if t.is_none() && m.is_none() && n.is_none() {
            None
        } else {
            let t = const_unwrap_or(t, 0);
            let m = const_unwrap_or(m, 0);
            let n = const_unwrap_or(n, 0);
            Some(const_max(const_max(t, m), n))
        }
    }
}

impl Material {
    pub const fn new(
        albedo: Uniform,
        metallic_roughness_emission: Mre,
        normal_texture: Option<u8>,
        specular: f32,
    ) -> Self {
        Self {
            albedo,
            metallic_roughness_emission,
            normal_texture,
            specular,
        }
    }
    pub const fn albedo(&self) -> &Uniform {
        &self.albedo
    }
    pub const fn metallic_roughness_emission(&self) -> &Mre {
        &self.metallic_roughness_emission
    }
    pub const fn normal_texture(&self) -> Option<u8> {
        self.normal_texture
    }
    pub const fn specular(&self) -> f32 {
        self.specular
    }
    #[cfg(feature = "safe")]
    pub(crate) const fn max_texture_idx(&self) -> Option<u8> {
        let t = self.albedo.texture();
        let m = self.metallic_roughness_emission.texture();
        let n = self.normal_texture;
        if t.is_none() && m.is_none() && n.is_none() {
            None
        } else {
            let t = const_unwrap_or(t, 0);
            let m = const_unwrap_or(m, 0);
            let n = const_unwrap_or(n, 0);
            Some(const_max(const_max(t, m), n))
        }
    }
}

#[cfg(feature = "safe")]
const fn const_max(a: u8, b: u8) -> u8 {
    if a > b {
        a
    } else {
        b
    }
}

#[cfg(feature = "safe")]
const fn const_unwrap_or(i: Option<u8>, v: u8) -> u8 {
    match i {
        Some(x) => x,
        None => v,
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: Uniform::default(),
            specular: 0.5,
            normal_texture: None,
            metallic_roughness_emission: Mre::default(),
        }
    }
}
