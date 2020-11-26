struct Point<T, U> {
    x: T,
    y: U,
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

    let point = SimplePoint { x: 5, y: 10 };
    println!("p.x = {}", point.x);

    let both_float = SimplePoint { x: 1.0, y: 4.0 };
    println!(
        "the distance from origin is {}",
        both_float.distance_from_origin()
    );

    // point.distance_from_origin();
}
