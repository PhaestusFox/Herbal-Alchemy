use strum_macros::EnumString;

#[derive(Debug, Hash, PartialEq, Clone, Copy, Default, EnumString, Eq)]
pub enum MeshTextureUVS {
    Legs = 0,
    #[default]
    Pot = 1,
    Wood = 2,
    Sand = 7,
    Water = 8,
    PalmTrunk = 9,
    PalmLeaf = 10,
    PalmNut = 11,
    PalmSeed = 12,
}

impl bevy_wave_collapse::vertex::VertexUV for MeshTextureUVS {
    fn to_f32x2(&self) -> [f32; 2] {
        (*self as u8).to_f32x2()
    }
}
