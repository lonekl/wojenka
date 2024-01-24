pub trait Overdraw<Another> {
    fn overdraw_on(&self, rhs: &mut Another);
}



#[derive(Clone, Copy, PartialEq)]
pub struct Rgb8 {

    pub r: u8,
    pub g: u8,
    pub b: u8,

}

impl Rgb8 {

    pub fn new(r: u8, g: u8, b: u8) -> Self {

        Self { r, g, b }
    }

}

impl Overdraw<Rgb8> for Rgb8 {
    fn overdraw_on(&self, rhs: &mut Rgb8) {
        *rhs = *self;
    }
}

impl Overdraw<Rgba8> for Rgb8 {
    fn overdraw_on(&self, rhs: &mut Rgba8) {
        *rhs = (*self).into();
    }
}

impl Into<Rgba8> for Rgb8 {
    fn into(self) -> Rgba8 {

        Rgba8::new(self.r, self.g, self.b, u8::MAX)
    }
}



#[derive(Clone, Copy, PartialEq)]
pub struct Rgba8 {

    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,

}

impl Rgba8 {

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {

        Self { r, g, b, a }
    }


    pub fn weighed_r(&self) -> u16 {

        self.r as u16 * self.a as u16
    }

    pub fn weighed_g(&self) -> u16 {

        self.g as u16 * self.a as u16
    }

    pub fn weighed_b(&self) -> u16 {

        self.b as u16 * self.a as u16
    }

}

impl Overdraw<Rgb8> for Rgba8 {
    fn overdraw_on(&self, rhs: &mut Rgb8) {

        rhs.r = ((self.r as i16  -  rhs.r as i16)  *  self.a as i16  /  0xff  +  rhs.r as i16) as u8;
        rhs.g = ((self.g as i16  -  rhs.g as i16)  *  self.a as i16  /  0xff  +  rhs.g as i16) as u8;
        rhs.b = ((self.b as i16  -  rhs.b as i16)  *  self.a as i16  /  0xff  +  rhs.b as i16) as u8;

    }
}

impl Overdraw<Rgba8> for Rgba8 {
    fn overdraw_on(&self, rhs: &mut Rgba8) {

        rhs.r = ((self.r as i16  -  rhs.r as i16)  *  self.a as i16  /  0xff  +  rhs.r as i16) as u8;
        rhs.g = ((self.g as i16  -  rhs.g as i16)  *  self.a as i16  /  0xff  +  rhs.g as i16) as u8;
        rhs.b = ((self.b as i16  -  rhs.b as i16)  *  self.a as i16  /  0xff  +  rhs.b as i16) as u8;
        rhs.a = self.a.checked_add(self.a).unwrap_or(u8::MAX);
    }
}

impl Into<Rgb8> for Rgba8 {
    fn into(self) -> Rgb8 {

        Rgb8::new(self.r, self.g, self.b)
    }
}
