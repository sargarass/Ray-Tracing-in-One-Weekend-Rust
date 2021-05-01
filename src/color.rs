use rand::distributions::Distribution;
use rand_distr::Uniform;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use rand::Rng;

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

    pub fn random<R: Rng>(rng: &mut R) -> Color {
        Color(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_minmax<R: Rng>(rng: &mut R, min: f32, max: f32) -> Color {
        assert!(min < max);
        let uniform_range = Uniform::from(min..max);
        Color(
            uniform_range.sample(rng),
            uniform_range.sample(rng),
            uniform_range.sample(rng),
        )
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn color_zero() {
        let a = Color::zero();
        assert_eq!(a.0, 0.0);
        assert_eq!(a.1, 0.0);
        assert_eq!(a.2, 0.0);
    }

    #[test]
    fn color_getter() {
        let a = Color(1.0, 2.0, 3.0);
        assert_eq!(a.r(), 1.0);
        assert_eq!(a.g(), 2.0);
        assert_eq!(a.b(), 3.0);
    }

    #[test]
    #[should_panic]
    fn color_lerp_exceed_left_bound() {
        let a = Color(1.0, 2.0, 3.0);
        a.lerp(a, -0.1);
    }
    #[test]
    #[should_panic]
    fn color_lerp_exceed_right_bound() {
        let a = Color(1.0, 2.0, 3.0);
        a.lerp(a, 1.1);
    }

    #[test]
    fn color_addsub() {
        let a = Color(1.0, 2.0, 3.0);
        let b = Color(-1.0, -2.0, -3.0);

        assert_eq!(a + b, Color(0.0, 0.0, 0.0));
        assert_eq!(a - b, Color(2.0, 4.0, 6.0));
    }

    #[test]
    fn color_mul() {
        let a = Color(1.0, 2.0, 3.0);
        let b = Color(-1.0, -2.0, -3.0);

        assert_eq!(a * b, Color(-1.0, -4.0, -9.0));
        assert_eq!(2.0 * a, Color(2.0, 4.0, 6.0));
    }

    #[test]
    fn color_div() {
        let a = Color(2.0, 4.0, 8.0);
        assert_eq!(a / 2.0, Color(1.0, 2.0, 4.0));
    }

    #[test]
    #[should_panic]
    fn color_random_min_eq_max() {
        Color::random_minmax(&mut thread_rng(), 1.0, 1.0);
    }

    #[test]
    #[should_panic]
    fn color_random_min_exceed_max() {
        Color::random_minmax(&mut thread_rng(), 2.0, 1.0);
    }
}