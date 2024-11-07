#[derive(Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Mesh {
    positions: Vec<[f32; 3]>,
    uvs: Option<Vec<[f32; 2]>>,
    normals: Option<Vec<[f32; 3]>>,
    colors: Option<Vec<[f32; 3]>>,
    tangents: Option<Vec<[f32; 4]>>,
    indices: Option<Vec<[u16; 3]>>,
    material: Option<u8>,
}

#[cfg(feature = "safe")]
impl ArchivedMesh {
    pub(crate) fn isnt_right(&self) -> bool {
        let l = self.positions.len();
        l != self.uvs.as_ref().map(|x| x.len()).unwrap_or(l)
            || l != self.normals.as_ref().map(|x| x.len()).unwrap_or(l)
            || l != self.colors.as_ref().map(|x| x.len()).unwrap_or(l)
            || l != self.tangents.as_ref().map(|x| x.len()).unwrap_or(l)
            || self
                .tangents
                .as_ref()
                .map(|vec| {
                    vec.into_iter()
                        .any(|[_, _, _, w]| f32::from(w).abs() != 1.0)
                })
                .unwrap_or(false)
    }
    pub(crate) const fn material(&self) -> Option<u8> {
        match self.material {
            rkyv::option::ArchivedOption::None => None,
            rkyv::option::ArchivedOption::Some(x) => Some(x),
        }
    }
}

impl Mesh {
    pub fn new(
        positions: Vec<[f32; 3]>,
        uvs: Option<Vec<[f32; 2]>>,
        normals: Option<Vec<[f32; 3]>>,
        colors: Option<Vec<[f32; 3]>>,
        tangents: Option<Vec<[f32; 4]>>,
        indices: Option<Vec<[u16; 3]>>,
        material: Option<u8>,
    ) -> Option<Self> {
        let ret = Self {
            positions,
            uvs,
            normals,
            colors,
            tangents,
            indices,
            material,
        };
        #[cfg(feature = "safe")]
        if ret.isnt_right() {
            None
        } else {
            Some(ret)
        }
        #[cfg(not(feature = "safe"))]
        Some(ret)
    }
    pub fn positions(&self) -> &[[f32; 3]] {
        &self.positions
    }
    pub fn uvs(&self) -> Option<&[[f32; 2]]> {
        self.uvs.as_deref()
    }
    pub fn normals(&self) -> Option<&[[f32; 3]]> {
        self.normals.as_deref()
    }
    pub fn colors(&self) -> Option<&[[f32; 3]]> {
        self.colors.as_deref()
    }
    pub fn tangents(&self) -> Option<&[[f32; 4]]> {
        self.tangents.as_deref()
    }
    pub fn indices(&self) -> Option<&[[u16; 3]]> {
        self.indices.as_deref()
    }
    pub const fn material(&self) -> Option<u8> {
        self.material
    }
    #[cfg(feature = "safe")]
    pub(crate) fn isnt_right(&self) -> bool {
        let l = self.positions.len();
        l != self.uvs.as_ref().map(|x| x.len()).unwrap_or(l)
            || l != self.normals.as_ref().map(|x| x.len()).unwrap_or(l)
            || l != self.colors.as_ref().map(|x| x.len()).unwrap_or(l)
            || l != self.tangents.as_ref().map(|x| x.len()).unwrap_or(l)
            || self
                .tangents
                .as_ref()
                .map(|vec| vec.into_iter().any(|[_, _, _, w]| w.abs() != 1.0))
                .unwrap_or(false)
    }
}
