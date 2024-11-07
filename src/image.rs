#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Image: [{}x{}]", self.width, self.height))
    }
}

impl Image {
    pub fn from_raw(width: u32, height: u32, data: &[u8]) -> Result<Self, qoi::Error> {
        let data = qoi::encode_to_vec(data, width, height)?;
        Ok(Self {
            width,
            height,
            data,
        })
    }
    pub const fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }
    pub const fn width(&self) -> u32 {
        self.width
    }
    pub const fn height(&self) -> u32 {
        self.height
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn decode(&self) -> Result<Vec<u8>, qoi::Error> {
        qoi::decode_to_vec(&self.data).map(|x| x.1)
    }
}
