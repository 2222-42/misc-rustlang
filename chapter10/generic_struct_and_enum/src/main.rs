use std::ops::Add;

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

#[derive(Debug, PartialEq)]
struct SimplePoint<T> {
    x: T,
    y: T,
}

impl Add for SimplePoint<i32> {
    type Output = SimplePoint<i32>;
    fn add(self, other: SimplePoint<i32>) -> SimplePoint<i32> {
        SimplePoint::<i32> {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
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

    let Point { x: a, y: b } = both_integer;
    assert_eq!(5, a);
    assert_eq!(10, b);
    let Point { x, y } = both_integer;
    assert_eq!(5, x);
    assert_eq!(10, y);
    match both_integer {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neigher axis: ({},{})", x, y),
    }

    match both_integer {
        Point { x, .. } => println!("x is {}", x),
    }

    assert_eq!(
        SimplePoint { x: 1, y: 0 } + SimplePoint { x: 2, y: 3 },
        SimplePoint { x: 3, y: 3 }
    );

    let both_float = Point { x: 1.0, y: 4.0 };
    let _integer_and_float = Point { x: 5, y: 4.0 };

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

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neigher axis: ({},{})", x, y),
    }

    let ((_feet, _inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    let points = vec![
        &Point { x: 0, y: 0 },
        &Point { x: 1, y: 5 },
        &Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|Point { x, y }| x * x + y * y).sum();
    println!("sum of squares: {}", sum_of_squares);
}
