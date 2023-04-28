use super::*;
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(Debug, Component, Hash, PartialEq, Eq)]
pub struct DynamicIsland;

#[derive(Debug, EnumIter, IntoStaticStr, Hash, Clone, Copy)]
enum IslandComponet {
    Core,
    ConnectWater,
    ConnectSand,
    ConnectWaterWater,
    ConnectSandSand,
    ConnectSandWater,
    ConnectWaterSand,
}

impl DynamicIsland {
    pub fn new<const NAME: usize>(asset_server: &AssetServer, path: &str) -> WaveObject {
        let mut meshes: HashMap<Connection, Handle<WaveMesh>> = HashMap::new();
        for component in IslandComponet::iter() {
            meshes.insert(
                Connection::new(component),
                asset_server.load(format!("{}#{:?}", path, component)),
            );
        }

        WaveObject {
            meshes,
            build_fn: DynamicIsland::build::<NAME>,
            can_connect_fn: |_| false,
        }
    }

    fn build<const NAME: usize>(
        obj: &WaveObject,
        offset: RVec3,
        assets: &Assets<WaveMesh>,
        main_mesh: &mut WaveBuilder,
        neighbors: &WaveData,
    ) -> Result<(), BakeError> {
        use IslandComponet::*;
        main_mesh.bake(
            offset,
            assets
                .get(obj.get(Core).ok_or(BakeError::MeshNotSet {
                    mesh: "Core",
                    obj: NAMES[NAME],
                })?)
                .ok_or(BakeError::MeshNotFound {
                    mesh: "Core",
                    obj: NAMES[NAME],
                })?,
        )?;
        let mut is_water = [true; 6];
        for i in 0..6 {
            is_water[i] = match neighbors[i] {
                MapCell::Water => true,
                MapCell::Sand => false,
            }
        }
        for i in 0..6 {
            let stright = if is_water[(6 - i) % 6] {
                obj.get(ConnectWater).ok_or(BakeError::MeshNotSet {
                    mesh: "Stright Water",
                    obj: NAMES[NAME],
                })?
            } else {
                obj.get(ConnectSand).ok_or(BakeError::MeshNotSet {
                    mesh: "Stright Flat",
                    obj: NAMES[NAME],
                })?
            };
            let mut stright = assets
                .get(stright)
                .ok_or(BakeError::MeshNotFound {
                    mesh: "Stright",
                    obj: NAMES[NAME],
                })?
                .clone();
            let cos = FixedPoint::ROTATIONS_COS[i];
            let sin = FixedPoint::ROTATIONS_SIN[i];
            stright.rotate(sin, cos);
            main_mesh.bake(offset, &stright)?;
            let corner = match (is_water[(6 - i) % 6], is_water[(5 - i) % 6]) {
                (true, true) => obj.get(ConnectWaterWater).ok_or(BakeError::MeshNotSet {
                    mesh: "Corner Water Water",
                    obj: NAMES[NAME],
                })?,
                (true, false) => obj.get(ConnectWaterSand).ok_or(BakeError::MeshNotSet {
                    mesh: "Corner Water Flat",
                    obj: NAMES[NAME],
                })?,
                (false, true) => obj.get(ConnectSandWater).ok_or(BakeError::MeshNotSet {
                    mesh: "Corner Flat Water",
                    obj: NAMES[NAME],
                })?,
                (false, false) => obj.get(ConnectSandSand).ok_or(BakeError::MeshNotSet {
                    mesh: "Corner Flat Flat",
                    obj: NAMES[NAME],
                })?,
            };
            let mut corner = assets
                .get(corner)
                .ok_or(BakeError::MeshNotFound {
                    mesh: "Corner",
                    obj: NAMES[NAME],
                })?
                .clone();
            let cos = FixedPoint::ROTATIONS_COS[i];
            let sin = FixedPoint::ROTATIONS_SIN[i];
            corner.rotate(sin, cos);
            main_mesh.bake(offset, &corner)?;
        }
        Ok(())
    }
}

impl Into<std::borrow::Cow<'static, str>> for IslandComponet {
    fn into(self) -> std::borrow::Cow<'static, str> {
        let val: &'static str = self.into();
        std::borrow::Cow::Borrowed(val)
    }
}

#[derive(Debug, Component, Hash, PartialEq, Eq)]
pub struct StaticIsland;

impl StaticIsland {
    pub fn new<const NAME: usize>(asset_server: &AssetServer, path: &str) -> WaveObject {
        let mut meshes: HashMap<Connection, Handle<WaveMesh>> = HashMap::new();
        meshes.insert(
            Connection::new(IslandComponet::Core),
            asset_server.load(format!("{}#Core", path)),
        );

        WaveObject {
            meshes,
            build_fn: StaticIsland::build::<NAME>,
            can_connect_fn: |_| false,
        }
    }
    fn build<const NAME: usize>(
        obj: &WaveObject,
        offset: RVec3,
        assets: &Assets<WaveMesh>,
        main_mesh: &mut WaveBuilder,
        _: &WaveData,
    ) -> Result<(), BakeError> {
        main_mesh.bake(
            offset,
            assets
                .get(obj.get(IslandComponet::Core).ok_or(BakeError::MeshNotSet {
                    mesh: "Core",
                    obj: NAMES[NAME],
                })?)
                .ok_or(BakeError::MeshNotFound {
                    mesh: "Core",
                    obj: NAMES[NAME],
                })?,
        )
    }
}
