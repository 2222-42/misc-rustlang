fn main() {
    let array1 = [15, 3, 5, 4, 10, 20];
    let length = array1.len();
    let mut sum = 0;
    let mut vector = Vec::new();
    for elem in array1.iter() {
        vector.push(elem)
    }
    for elem in vector.iter() {
        sum += *elem
    }
    let mean = sum / length;
    println!("mean is {}", mean);

    vector.sort();
    let median = vector[(length / 2) - 1];
    println!("median is {}", median);
}
