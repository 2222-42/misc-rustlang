use std::ops::{Add, AddAssign, Neg};

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

fn main() {
    println!("Hello, world!");
}
