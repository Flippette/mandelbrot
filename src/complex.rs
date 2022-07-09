use std::{cmp::Ordering, fmt, ops};

#[derive(Debug, Clone, Copy)]
pub struct Complex(pub f64, pub f64);

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

impl ops::Add<f64> for Complex {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs, self.1)
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Self;

    fn add(self, rhs: Complex) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl ops::Sub<f64> for Complex {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs, self.1)
    }
}

impl ops::Sub<Complex> for Complex {
    type Output = Self;

    fn sub(self, rhs: Complex) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl ops::Mul<Complex> for Complex {
    type Output = Self;

    fn mul(self, rhs: Complex) -> Self::Output {
        Self(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl ops::Div<f64> for Complex {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl ops::Div<Complex> for Complex {
    type Output = Self;

    fn div(self, rhs: Complex) -> Self::Output {
        Self(
            (self.0 * rhs.0 + self.1 * rhs.1) / (rhs.0 * rhs.0 + rhs.1 * rhs.1),
            (self.1 * rhs.0 - self.0 * rhs.1) / (rhs.0 * rhs.0 + rhs.1 * rhs.1),
        )
    }
}

impl ops::AddAssign<f64> for Complex {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
    }
}

impl ops::AddAssign<Complex> for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl ops::SubAssign<f64> for Complex {
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
    }
}

impl ops::SubAssign<Complex> for Complex {
    fn sub_assign(&mut self, rhs: Complex) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl ops::MulAssign<f64> for Complex {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl ops::MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, rhs: Complex) {
        self.0 = self.0 * rhs.0 - self.1 * rhs.1;
        self.1 = self.0 * rhs.1 + self.1 * rhs.0;
    }
}

impl ops::DivAssign<f64> for Complex {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

impl ops::DivAssign<Complex> for Complex {
    fn div_assign(&mut self, rhs: Complex) {
        self.0 = (self.0 * rhs.0 + self.1 * rhs.1) / (rhs.0 * rhs.0 + rhs.1 * rhs.1);
        self.1 = (self.1 * rhs.0 - self.0 * rhs.1) / (rhs.0 * rhs.0 + rhs.1 * rhs.1);
    }
}
