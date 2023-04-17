use crate::prelude::*;
use bevy::{
    asset::{AssetLoader, HandleId, LoadedAsset},
    prelude::*,
};

#[derive(Resource)]
pub struct VoidHandles {
    _handles: Vec<HandleUntyped>,
}

impl FromWorld for VoidHandles {
    fn from_world(world: &mut World) -> Self {
        let mut data = Vec::with_capacity(1);
        world.resource_scope(
            |world: &mut World, mut matts: Mut<Assets<CustomMaterial>>| {
                let asset_server = world.resource::<AssetServer>();
                data.push(
                    matts
                        .set(
                            ConstHandles::WaveMaterial,
                            CustomMaterial {
                                // unlit: true,
                                base_color_texture: Some(asset_server.load("textures/mesh.png")),
                                ..Default::default()
                            },
                        )
                        .into(),
                );
            },
        );
        VoidHandles { _handles: data }
    }
}

pub enum ConstHandles {
    WaveMaterial = 0,
}

impl Into<HandleId> for ConstHandles {
    fn into(self) -> HandleId {
        HandleId::Id(
            uuid::uuid!("c329f1c4-7eaf-497a-80da-bb4717ea50b9"),
            self as u64,
        )
    }
}

#[derive(Default)]
pub struct ObjLoader;

impl AssetLoader for ObjLoader {
    fn extensions(&self) -> &[&str] {
        &["obj"]
    }
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async {
            let wave = WaveMesh::from_obj_str(String::from_utf8_lossy(bytes).as_ref())?;
            for (label, mesh) in wave.into_iter() {
                load_context.set_labeled_asset(
                    &label,
                    LoadedAsset::new(mesh.extract_mesh(
                        bevy::render::render_resource::PrimitiveTopology::TriangleList,
                    )),
                );
            }
            Ok(())
        })
    }
}
