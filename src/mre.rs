#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum Mre {
    Texture(u8),
    Value {
        metallic: f32,
        roughness: f32,
        emission: f32,
    },
}

impl std::fmt::Debug for Mre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Texture(x) => f.write_fmt(format_args!("Texture({x})")),
            Self::Value {
                metallic: m,
                roughness: r,
                emission: e,
            } => f.write_fmt(format_args!(
                "(metallic: {m}, roughness: {r}, emission: {e})"
            )),
        }
    }
}

#[cfg(feature = "safe")]
impl ArchivedMre {
    pub(crate) const fn texture(&self) -> Option<u8> {
        match self {
            Self::Texture(t) => Some(*t),
            _ => None,
        }
    }
}

impl Mre {
    pub const fn texture(&self) -> Option<u8> {
        match self {
            Self::Texture(t) => Some(*t),
            _ => None,
        }
    }
    pub const fn metallic(&self) -> Option<f32> {
        match self {
            Self::Value { metallic: x, .. } => Some(*x),
            _ => None,
        }
    }
    pub const fn roughness(&self) -> Option<f32> {
        match self {
            Self::Value { roughness: x, .. } => Some(*x),
            _ => None,
        }
    }
    pub const fn emission(&self) -> Option<f32> {
        match self {
            Self::Value { emission: x, .. } => Some(*x),
            _ => None,
        }
    }
}

impl Default for Mre {
    fn default() -> Self {
        Self::Value {
            metallic: 0.0,
            roughness: 0.5,
            emission: 0.0,
        }
    }
}
