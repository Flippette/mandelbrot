use std::ops;

#[cfg(all(feature = "single", not(feature = "double")))]
pub type Float = f32;

#[cfg(all(feature = "double", not(feature = "single")))]
pub type Float = f64;

#[cfg(all(feature = "double", feature = "single"))]
compile_error!("cannot enable both features \"double\" and \"single\" at the same time!");

#[derive(Clone, Copy)]
pub struct Complex(pub Float, pub Float);

impl ops::Add<Complex> for Complex {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Complex) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Complex {
    #[inline(always)]
    pub fn sqr(self) -> Self {
        Self(self.0 * self.0 - self.1 * self.1, 2.0 * self.0 * self.1)
    }
}
