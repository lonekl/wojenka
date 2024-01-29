#[derive(Clone, Copy)]
pub struct TerrainPart ( u16);

impl TerrainPart {

    pub fn from_16bit_scale(part: u16) -> Self {

        Self ( part)
    }

    pub fn from_8bit_scale(part: u8) -> Self {

        Self ( (part as u16) << 8)
    }



    pub fn rescale_texture_levels(self, max: u8) -> u8 {

        (self.0 as u32  *  max as u32  /  u16::MAX as u32) as u8
    }

}



#[derive(Clone, Copy)]
pub struct TerrainHeight ( i32);

impl TerrainHeight {

    pub fn from_meters(altitude: i32) -> Self {

        Self ( altitude)
    }

    pub fn to_meters(self) -> i32 {

        self.0
    }

    pub fn to_10km_f32(self) -> f32 {

        self.0 as f32 / 10_000.0
    }

}



#[derive(Clone, Copy)]
pub struct HeightVariation ( u32);

impl HeightVariation {

    pub fn from_meters(from_average: u32) -> Self {

        Self ( from_average)
    }

}



#[derive(Clone, Copy)]
pub struct Distance ( u32);

impl Distance {

    pub fn from_meters(distance: u32) -> Self {

        Self ( distance)
    }

    pub fn from_kilometers(distance: u32) -> Self {

        Self ( distance * 1000)
    }

}
