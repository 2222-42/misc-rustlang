#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct SimplePoint<T> {
    x: T,
    y: T,
}

impl<T> SimplePoint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl SimplePoint<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    let new_pointer = both_integer.mixup(both_float);
    println!("{:?}", new_pointer);

    let point = SimplePoint { x: 5, y: 10 };
    println!("p.x = {}", point.x);

    let both_float = SimplePoint { x: 1.0, y: 4.0 };
    println!(
        "the distance from origin is {}",
        both_float.distance_from_origin()
    );

    // point.distance_from_origin();
}
