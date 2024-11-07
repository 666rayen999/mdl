#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub(crate) enum FileData {
    Compressed(Vec<u8>),
    Uncompressed(Vec<u8>),
}

#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub(crate) struct File {
    magic: String,
    data: FileData,
}

impl File {
    pub(crate) fn save(bytes: &[u8]) -> Self {
        #[cfg(feature = "compression")]
        {
            let c = lz4_flex::block::compress_prepend_size(bytes);
            if c.len() < bytes.len() {
                Self {
                    magic: "RNM".into(),
                    data: FileData::Compressed(c),
                }
            } else {
                Self {
                    magic: "RNM".into(),
                    data: FileData::Uncompressed(bytes.to_vec()),
                }
            }
        }
        #[cfg(not(feature = "compression"))]
        {
            Self {
                magic: "RNM".into(),
                data: FileData::Uncompressed(bytes.to_vec()),
            }
        }
    }
    pub(crate) fn load(file: &ArchivedFile) -> Option<Vec<u8>> {
        if file.magic != "RNM" {
            None
        } else {
            match &file.data {
                #[cfg(feature = "compression")]
                ArchivedFileData::Compressed(x) => {
                    lz4_flex::block::decompress_size_prepended(&x).ok()
                }
                #[cfg(not(feature = "compression"))]
                ArchivedFileData::Compressed(_) => {
                    println!("[ERROR]: compression feature is disabled!");
                    None
                }
                ArchivedFileData::Uncompressed(x) => Some(x.to_vec()),
            }
        }
    }
}
