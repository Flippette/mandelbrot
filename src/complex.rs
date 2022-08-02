use std::{cmp::Ordering, fmt, ops};

#[cfg(all(feature="single", not(feature="double")))]
pub type Float = f32;

#[cfg(all(feature="double", not(feature="single")))]
pub type Float = f64;

#[cfg(all(feature="double", feature="single"))]
compile_error!("Feature \"double\" and \"single\" are mutually exclusive!");

#[derive(Debug, Clone, Copy)]
pub struct Complex(pub Float, pub Float);

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let real = self.0;
        let imaginary = self.1;
        f.write_str(&format!(
            "{}{}",
            real,
            match imaginary.partial_cmp(&0.0) {
                Some(order) if order == Ordering::Less => format!(" - {}i", -imaginary),
                Some(order) if order == Ordering::Equal => "".to_owned(),
                _ => format!(" + {}i", imaginary),
            }
        ))
    }
}

impl ops::Add<Float> for Complex {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Float) -> Self::Output {
        Self(self.0 + rhs, self.1)
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Complex) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Neg for Complex {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl ops::Sub<Float> for Complex {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Float) -> Self::Output {
        Self(self.0 - rhs, self.1)
    }
}

impl ops::Sub<Complex> for Complex {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Complex) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul<Float> for Complex {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Float) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl ops::Mul<Complex> for Complex {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Complex) -> Self::Output {
        Self(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl ops::Div<Float> for Complex {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Float) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl ops::Div<Complex> for Complex {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Complex) -> Self::Output {
        Self(
            (self.0 * rhs.0 + self.1 * rhs.1) / (rhs.0 * rhs.0 + rhs.1 * rhs.1),
            (self.1 * rhs.0 - self.0 * rhs.1) / (rhs.0 * rhs.0 + rhs.1 * rhs.1),
        )
    }
}

impl ops::AddAssign<Float> for Complex {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Float) {
        self.0 += rhs;
    }
}

impl ops::AddAssign<Complex> for Complex {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Complex) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl ops::SubAssign<Float> for Complex {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Float) {
        self.0 -= rhs;
    }
}

impl ops::SubAssign<Complex> for Complex {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Complex) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl ops::MulAssign<Float> for Complex {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Float) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl ops::MulAssign<Complex> for Complex {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Complex) {
        self.0 = self.0 * rhs.0 - self.1 * rhs.1;
        self.1 = self.0 * rhs.1 + self.1 * rhs.0;
    }
}

impl ops::DivAssign<Float> for Complex {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Float) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

impl ops::DivAssign<Complex> for Complex {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Complex) {
        self.0 = (self.0 * rhs.0 + self.1 * rhs.1) / (rhs.0 * rhs.0 + rhs.1 * rhs.1);
        self.1 = (self.1 * rhs.0 - self.0 * rhs.1) / (rhs.0 * rhs.0 + rhs.1 * rhs.1);
    }
}
