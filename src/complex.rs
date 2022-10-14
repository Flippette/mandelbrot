use std::ops;

#[derive(Clone, Copy)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl ops::Add<Complex> for Complex {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Complex) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    #[inline(always)]
    pub fn sqr(self) -> Self {
        Self::new(
            self.re * self.re - self.im * self.im,
            2.0 * self.re * self.im,
        )
    }
}
