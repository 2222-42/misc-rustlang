fn main() {
    let x = 5;

    let (x, y, z) = (1, 2, 3);

    // let (x, y) = (1, 2, 3);
    let (x, y, _) = (1, 2, 3);

    let (x, ..) = (1, 2, 3);
}
