#[derive(Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum Light {
    Point {
        position: [f32; 3],
        color: [f32; 3],
        power: f32,
        radius: f32,
    },
    Sun {
        direction: [f32; 3],
        color: [f32; 3],
        power: f32,
    },
    Spot {
        position: [f32; 3],
        direction: [f32; 3],
        color: [f32; 3],
        power: f32,
        radius: f32,
        blend: f32,
    },
}

impl Light {
    pub const fn color(&self) -> &[f32; 3] {
        match self {
            Self::Point { color, .. } => color,
            Self::Sun { color, .. } => color,
            Self::Spot { color, .. } => color,
        }
    }
    pub const fn power(&self) -> f32 {
        match self {
            Self::Point { power, .. } => *power,
            Self::Sun { power, .. } => *power,
            Self::Spot { power, .. } => *power,
        }
    }
}
