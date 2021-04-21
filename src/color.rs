use std::ops::{Add, Mul, AddAssign, Div};

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

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
