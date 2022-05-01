use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl<P> Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

fn main() {
    let mut image: Image<i32> = Image::new(100, 50);
    println!("{:?}", image[0][99]);
    image[0][99] = 42;
    println!("{:?}", image[0][99]);
}
