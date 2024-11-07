#[derive(Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum Uniform {
    Color([f32; 3]),
    Texture(u8),
}

#[cfg(feature = "safe")]
impl ArchivedUniform {
    pub(crate) const fn texture(&self) -> Option<u8> {
        match self {
            Self::Texture(t) => Some(*t),
            _ => None,
        }
    }
}

impl Uniform {
    pub const fn new_color(color: [f32; 3]) -> Self {
        Self::Color(color)
    }
    pub const fn new_texture(data: u8) -> Self {
        Self::Texture(data)
    }
    pub const fn color(&self) -> Option<&[f32; 3]> {
        match self {
            Self::Color(c) => Some(c),
            _ => None,
        }
    }
    pub const fn texture(&self) -> Option<u8> {
        match self {
            Self::Texture(t) => Some(*t),
            _ => None,
        }
    }
}

impl Default for Uniform {
    fn default() -> Self {
        Self::Color([1.0; 3])
    }
}
