use crate::{file::ArchivedFile, file::File, Image, Light, Material, Mesh};

#[cfg(feature = "safe")]
use crate::{material::ArchivedMaterial, mesh::ArchivedMesh};

#[derive(Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Scene {
    meshes: Option<Vec<Mesh>>,
    materials: Option<Vec<Material>>,
    lights: Option<Vec<Light>>,
    images: Option<Vec<Image>>,
}

impl Scene {
    pub fn meshes(&self) -> Option<&[Mesh]> {
        self.meshes.as_deref()
    }
    pub fn materials(&self) -> Option<&[Material]> {
        self.materials.as_deref()
    }
    pub fn lights(&self) -> Option<&[Light]> {
        self.lights.as_deref()
    }
    pub fn images(&self) -> Option<&[Image]> {
        self.images.as_deref()
    }
    pub fn decode_images(&self) -> Option<Vec<Vec<u8>>> {
        self.images
            .as_ref()
            .map(|images| {
                images
                    .into_iter()
                    .map(Image::decode)
                    .map(Result::ok)
                    .collect()
            })
            .flatten()
    }
    fn add<T>(items: &mut Option<Vec<T>>, item: T) {
        if let Some(items) = items {
            items.push(item);
        } else {
            *items = Some(vec![item]);
        }
    }
    pub fn add_mesh(&mut self, mesh: Mesh) {
        Self::add(&mut self.meshes, mesh);
    }
    pub fn add_material(&mut self, material: Material) {
        Self::add(&mut self.materials, material);
    }
    pub fn add_light(&mut self, light: Light) {
        Self::add(&mut self.lights, light);
    }
    pub fn add_image(&mut self, image: Image) {
        Self::add(&mut self.images, image);
    }
    pub fn new(
        meshes: Option<Vec<Mesh>>,
        materials: Option<Vec<Material>>,
        lights: Option<Vec<Light>>,
        images: Option<Vec<Image>>,
    ) -> Option<Self> {
        #[cfg(feature = "safe")]
        {
            let tex = images.as_ref().map(Vec::len).unwrap_or(0);
            let mat = materials.as_ref().map(Vec::len).unwrap_or(0);
            if !materials
                .as_ref()
                .map(|m| {
                    m.iter()
                        .filter_map(Material::max_texture_idx)
                        .max()
                        .map(|x| x as usize)
                        .map(|max| max < tex)
                })
                .flatten()
                .unwrap_or(true)
            {
                println!("[ERROR]: images len is less than image index in material!");
                None
            } else if !meshes.as_ref().is_some_and(|m| {
                m.iter()
                    .filter_map(Mesh::material)
                    .map(|x| x as usize)
                    .all(|m| m < mat)
            }) {
                println!("[ERROR]: materials len is less than material index in mesh!");
                None
            } else if meshes
                .as_ref()
                .is_some_and(|m| m.iter().any(Mesh::isnt_right))
            {
                println!("[ERROR]: mesh attributes DOESNT have the same len!");
                None
            } else {
                Some(Self {
                    meshes,
                    materials,
                    lights,
                    images,
                })
            }
        }
        #[cfg(not(feature = "safe"))]
        {
            Some(Self {
                meshes,
                materials,
                lights,
                images,
            })
        }
    }
    pub fn load(data: &[u8]) -> Option<Self> {
        let mut bytes = rkyv::util::AlignedVec::<16>::new();
        bytes.extend_from_slice(data);
        #[cfg(feature = "safe")]
        {
            let file = File::load(
                rkyv::access::<ArchivedFile, rkyv::rancor::Error>(&bytes)
                    .map_err(|e| println!("[ERROR]: {e:?}"))
                    .ok()?,
            )?;
            let data = rkyv::access::<ArchivedScene, rkyv::rancor::Error>(&file)
                .map_err(|e| println!("[ERROR]: {e:?}"))
                .ok()?;
            let tex = data
                .images
                .as_ref()
                .map(rkyv::vec::ArchivedVec::len)
                .unwrap_or(0);
            let mat = data
                .materials
                .as_ref()
                .map(rkyv::vec::ArchivedVec::len)
                .unwrap_or(0);
            if !data
                .materials
                .as_ref()
                .map(|m| {
                    m.iter()
                        .filter_map(ArchivedMaterial::max_texture_idx)
                        .max()
                        .map(|x| x as usize)
                        .map(|max| max < tex)
                })
                .flatten()
                .unwrap_or(true)
            {
                println!("[ERROR]: images len is less than image index in material!");
                None
            } else if !data.meshes.as_ref().is_some_and(|m| {
                m.iter()
                    .filter_map(ArchivedMesh::material)
                    .map(|x| x as usize)
                    .all(|m| m < mat)
            }) {
                println!("[ERROR]: materials len is less than material index in mesh!");
                None
            } else if data
                .meshes
                .as_ref()
                .is_some_and(|m| m.iter().any(ArchivedMesh::isnt_right))
            {
                println!("[ERROR]: mesh attributes DOESNT have the same len!");
                None
            } else {
                Some(
                    rkyv::deserialize::<Self, rkyv::rancor::Error>(data)
                        .map_err(|e| println!("[ERROR]: {e:?}"))
                        .ok()?,
                )
            }
        }
        #[cfg(not(feature = "safe"))]
        {
            let file = File::load(unsafe { rkyv::access_unchecked::<ArchivedFile>(&bytes) })?;
            let data = unsafe { rkyv::access_unchecked::<ArchivedScene>(&file) };
            Some(
                rkyv::deserialize::<Self, rkyv::rancor::Error>(data)
                    .map_err(|e| println!("[ERROR]: {e:?}"))
                    .ok()?,
            )
        }
    }
    pub fn save(&self, path: &str) -> Result<(), ()> {
        use std::io::Write;
        let mut file = std::fs::File::create(path).map_err(|e| println!("[ERROR]: {e:?}"))?;
        let bytes =
            rkyv::to_bytes::<rkyv::rancor::Error>(self).map_err(|e| println!("[ERROR]: {e:?}"))?;
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&File::save(&bytes))
            .map_err(|e| println!("[ERROR]: {e:?}"))?;
        file.write_all(&bytes)
            .map_err(|e| println!("[ERROR]: {e:?}"))?;
        Ok(())
    }
}
