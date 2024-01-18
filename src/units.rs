use std::f32::consts::PI;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};



#[derive(Clone)]
pub struct RotationXYZ {

    pub x: Angle,
    pub y: Angle,
    pub z: Angle,

}

impl RotationXYZ {

    pub const ZERO: Self = Self::new(Angle::ZERO, Angle::ZERO, Angle::ZERO);

    pub const fn new(x: Angle, y: Angle, z: Angle) -> Self {

        Self { x, y, z }
    }

    pub fn rotation_matrix(&self) -> Matrix3x3 {

        self.x.rotation_matrix_x() * self.y.rotation_matrix_y() * self.z.rotation_matrix_z()
    }

}



#[derive(Clone, Copy)]
pub struct Angle ( f32);

impl Angle {

    pub const ZERO: Self = Self ( 0.0);

    const DEGREES_TO_RADIANS: f32 = PI / 180.0;
    const RADIANS_TO_DEGREES: f32 = 180.0 / PI;

    pub fn from_radians(radians: f32) -> Self {

        Self ( radians)
    }

    pub fn from_degrees(degrees: f32) -> Self {

        Self ( degrees * Self::DEGREES_TO_RADIANS)
    }


    pub fn sin(self) -> f32 {

        self.0.sin()
    }

    pub fn cos(self) -> f32 {

        self.0.cos()
    }

    pub fn tan(self) -> f32 {

        self.0.tan()
    }

    pub fn ctg(self) -> f32 {

        (self.0 + (PI / 2.0)).tan()
    }


    pub fn rotation_matrix_x(self) -> Matrix3x3 {

        Matrix3x3::from_arrays([
            [1.0,        0.0,         0.0],
            [0.0,  self.cos(),  self.sin()],
            [0.0, -self.sin(),  self.cos()],
        ])
    }

    pub fn rotation_matrix_y(self) -> Matrix3x3 {

        Matrix3x3::from_arrays([
            [ self.cos(), 0.0, self.sin()],
            [        0.0, 1.0,        0.0],
            [-self.sin(), 0.0, self.cos()],
        ])
    }

    pub fn rotation_matrix_z(self) -> Matrix3x3 {

        Matrix3x3::from_arrays([
            [1.0,         0.0,        0.0],
            [0.0,  self.cos(), self.sin()],
            [0.0, -self.sin(), self.cos()],
        ])
    }


    pub fn to_radians(self) -> f32 {

        self.0
    }

    pub fn to_degrees(self) -> f32 {

        self.0 * Self::RADIANS_TO_DEGREES
    }

}

impl Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Self::Output {

        Self ( self.0 + rhs.0)
    }
}

impl AddAssign<Angle> for Angle {
    fn add_assign(&mut self, rhs: Angle) {
        *self = *self + rhs;
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;

    fn sub(self, rhs: Angle) -> Self::Output {

        Self ( self.0 - rhs.0)
    }
}

impl SubAssign<Angle> for Angle {
    fn sub_assign(&mut self, rhs: Angle) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Angle {
    type Output = Angle;

    fn mul(self, rhs: f32) -> Self::Output {

        Self ( self.0 * rhs)
    }
}

impl MulAssign<f32> for Angle {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Angle {
    type Output = Angle;

    fn div(self, rhs: f32) -> Self::Output {

        Self ( self.0 / rhs)
    }
}

impl DivAssign<f32> for Angle {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}



pub struct Matrix3x3 ( [[f32; 3]; 3]);

impl Matrix3x3 {

    pub fn from_arrays(matrix: [[f32; 3]; 3]) -> Self {

        Self ( matrix)
    }

    pub fn to_arrays(self) -> [[f32; 3]; 3] {

        self.0
    }

}

impl Mul<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {

        Self ([
            [
                self.0 [0][0] * rhs.0 [0][0]  +  self.0 [0][1] * rhs.0 [1][0]  +  self.0 [0][2] * rhs.0 [2][0],
                self.0 [0][0] * rhs.0 [0][1]  +  self.0 [0][1] * rhs.0 [1][1]  +  self.0 [0][2] * rhs.0 [2][1],
                self.0 [0][0] * rhs.0 [0][2]  +  self.0 [0][1] * rhs.0 [1][2]  +  self.0 [0][2] * rhs.0 [2][2],
            ],
            [
                self.0 [1][0] * rhs.0 [0][0]  +  self.0 [1][1] * rhs.0 [1][0]  +  self.0 [1][2] * rhs.0 [2][0],
                self.0 [1][0] * rhs.0 [0][1]  +  self.0 [1][1] * rhs.0 [1][1]  +  self.0 [1][2] * rhs.0 [2][1],
                self.0 [1][0] * rhs.0 [0][2]  +  self.0 [1][1] * rhs.0 [1][2]  +  self.0 [1][2] * rhs.0 [2][2],
            ],
            [
                self.0 [2][0] * rhs.0 [0][0]  +  self.0 [2][1] * rhs.0 [1][0]  +  self.0 [2][2] * rhs.0 [2][0],
                self.0 [2][0] * rhs.0 [0][1]  +  self.0 [2][1] * rhs.0 [1][1]  +  self.0 [2][2] * rhs.0 [2][1],
                self.0 [2][0] * rhs.0 [0][2]  +  self.0 [2][1] * rhs.0 [1][2]  +  self.0 [2][2] * rhs.0 [2][2],
            ],
        ])
    }
}



#[derive(Clone)]
pub struct Matrix4x4 ( [[f32; 4]; 4]);

impl Matrix4x4 {

    pub fn from_arrays(matrix: [[f32; 4]; 4]) -> Self {

        Self ( matrix)
    }


    pub fn projection_matrix(window_dimensions: (u32, u32), fov_x: Angle, fov_y: Angle, z_near: f32, z_far: f32) -> Self {
        let aspect_ratio = window_dimensions.1 as f32 / window_dimensions.0 as f32;
        let fov_x_ratio = 1.0 / (fov_x / 2.0).tan();
        let fov_y_ratio = 1.0 / (fov_y / 2.0).tan();


        Self ( [
            [fov_x_ratio * aspect_ratio, 0.0,         0.0,                                        0.0],
            [0.0,                        fov_y_ratio, 0.0,                                        0.0],
            [0.0,                        0.0,         (z_far + z_near) / (z_far - z_near),        1.0],
            [0.0,                        0.0,         -(2.0 * z_far * z_near) / (z_far - z_near), 0.0],
        ])
    }


    pub fn to_arrays(self) -> [[f32; 4]; 4] {

        self.0
    }

}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {

        Self ([
            [
                self.0 [0][0] * rhs.0 [0][0]  +  self.0 [0][1] * rhs.0 [1][0]  +  self.0 [0][2] * rhs.0 [2][0]  +  self.0 [0][3] * rhs.0 [3][0],
                self.0 [0][0] * rhs.0 [0][1]  +  self.0 [0][1] * rhs.0 [1][1]  +  self.0 [0][2] * rhs.0 [2][1]  +  self.0 [0][3] * rhs.0 [3][1],
                self.0 [0][0] * rhs.0 [0][2]  +  self.0 [0][1] * rhs.0 [1][2]  +  self.0 [0][2] * rhs.0 [2][2]  +  self.0 [0][3] * rhs.0 [3][2],
                self.0 [0][0] * rhs.0 [0][3]  +  self.0 [0][1] * rhs.0 [1][3]  +  self.0 [0][2] * rhs.0 [2][3]  +  self.0 [0][3] * rhs.0 [3][3],
            ],
            [
                self.0 [1][0] * rhs.0 [0][0]  +  self.0 [1][1] * rhs.0 [1][0]  +  self.0 [1][2] * rhs.0 [2][0]  +  self.0 [1][3] * rhs.0 [3][0],
                self.0 [1][0] * rhs.0 [0][1]  +  self.0 [1][1] * rhs.0 [1][1]  +  self.0 [1][2] * rhs.0 [2][1]  +  self.0 [1][3] * rhs.0 [3][1],
                self.0 [1][0] * rhs.0 [0][2]  +  self.0 [1][1] * rhs.0 [1][2]  +  self.0 [1][2] * rhs.0 [2][2]  +  self.0 [1][3] * rhs.0 [3][2],
                self.0 [1][0] * rhs.0 [0][3]  +  self.0 [1][1] * rhs.0 [1][3]  +  self.0 [1][2] * rhs.0 [2][3]  +  self.0 [1][3] * rhs.0 [3][3],
            ],
            [
                self.0 [2][0] * rhs.0 [0][0]  +  self.0 [2][1] * rhs.0 [1][0]  +  self.0 [2][2] * rhs.0 [2][0]  +  self.0 [2][3] * rhs.0 [3][0],
                self.0 [2][0] * rhs.0 [0][1]  +  self.0 [2][1] * rhs.0 [1][1]  +  self.0 [2][2] * rhs.0 [2][1]  +  self.0 [2][3] * rhs.0 [3][1],
                self.0 [2][0] * rhs.0 [0][2]  +  self.0 [2][1] * rhs.0 [1][2]  +  self.0 [2][2] * rhs.0 [2][2]  +  self.0 [2][3] * rhs.0 [3][2],
                self.0 [2][0] * rhs.0 [0][3]  +  self.0 [2][1] * rhs.0 [1][3]  +  self.0 [2][2] * rhs.0 [2][3]  +  self.0 [2][3] * rhs.0 [3][3],
            ],
            [
                self.0 [3][0] * rhs.0 [0][0]  +  self.0 [3][1] * rhs.0 [1][0]  +  self.0 [3][2] * rhs.0 [2][0]  +  self.0 [3][3] * rhs.0 [3][0],
                self.0 [3][0] * rhs.0 [0][1]  +  self.0 [3][1] * rhs.0 [1][1]  +  self.0 [3][2] * rhs.0 [2][1]  +  self.0 [3][3] * rhs.0 [3][1],
                self.0 [3][0] * rhs.0 [0][2]  +  self.0 [3][1] * rhs.0 [1][2]  +  self.0 [3][2] * rhs.0 [2][2]  +  self.0 [3][3] * rhs.0 [3][2],
                self.0 [3][0] * rhs.0 [0][3]  +  self.0 [3][1] * rhs.0 [1][3]  +  self.0 [3][2] * rhs.0 [2][3]  +  self.0 [3][3] * rhs.0 [3][3],
            ],
        ])
    }

}



#[derive(Clone, Copy)]
pub struct Position {

    pub x: Length,
    pub y: Length,
    pub z: Length,

}

impl Position {

    pub fn new(x: Length, y: Length, z: Length) -> Self {

        Self {
            x,
            y,
            z,
        }
    }


    pub fn tuple(self) -> (Length, Length, Length) {

        (self.x, self.y, self.z)
    }

    pub fn array(self) -> [Length; 3] {

        [self.x, self.y, self.z]
    }


    pub fn unit_tuple(self) -> (f32, f32, f32) {

        (self.x.to_units(), self.y.to_units(), self.z.to_units())
    }

    pub fn unit_array(self) -> [f32; 3] {

        [self.x.to_units(), self.y.to_units(), self.z.to_units()]
    }

}


#[derive(Clone, Copy)]
pub struct Length ( f32);

impl Length {

    pub fn from_units(units: f32) -> Self {

        Self ( units)
    }

    pub fn from_10km(km_x10: f32) -> Self {

        Self ( km_x10)
    }

    pub fn from_1km(km: f32) -> Self {

        Self ( km * 0.1)
    }


    pub fn to_units(self) -> f32 {

        self.0
    }

    pub fn to_10km(self) -> f32 {

        self.0
    }

    pub fn to_1km(self) -> f32 {

        self.0 * 10.0
    }

}
