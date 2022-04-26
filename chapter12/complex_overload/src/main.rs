use std::ops::{Add, AddAssign, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Complex<T> {
    /// Real portion of the complex number
    pub re: T,
    /// Imaginary portion of the complex number
    pub im: T,
}

// impl<T> Add for Complex<T>
// where
//     T: Add<Output = T>,
// {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Self {
//             re: self.re + rhs.re,
//             im: self.im + rhs.im,
//         }
//     }
// }

impl<L, R> Add<Complex<R>> for Complex<L>
where
    L: Add<R>,
{
    type Output = Complex<L::Output>;
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Mul for Complex<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let re1 = self.re * rhs.re;
        let re2 = self.im * rhs.im;
        let re = re1 - re2;
        let im = self.re * rhs.im + self.im * rhs.re;
        Complex { re, im }
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>, // the output of negation should be the same type as the input
{
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

fn main() {
    let x = Complex { re: 5, im: 2 };
    let y = Complex { re: 2, im: 5 };
    assert_eq!(x * y, Complex { re: 0, im: 29 });
}
