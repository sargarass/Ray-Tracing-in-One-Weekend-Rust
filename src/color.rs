use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Color(pub f32, pub f32, pub f32);

impl Color {
    pub fn r(&self) -> f32 {
        self.0
    }
    pub fn g(&self) -> f32 {
        self.1
    }
    pub fn b(&self) -> f32 {
        self.2
    }

    pub fn zero() -> Self {
        Color(0.0, 0.0, 0.0)
    }

    pub fn lerp(self, end: Color, t: f32) -> Color {
        assert!((0.0..=1.0).contains(&t));
        (1.0 - t) * self + t * end
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, mut rhs: Color) -> Self::Output {
        rhs *= self;
        rhs
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, mut rhs: Color) -> Self::Output {
        rhs *= self;
        rhs
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}
