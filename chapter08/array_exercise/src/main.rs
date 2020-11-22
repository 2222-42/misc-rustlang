use std::collections::HashMap;
fn main() {
    let array1 = [15, 10, 3, 3, 4, 5, 5, 10, 20, 20, 15, 2, 3, 4, 20, 20, 3];
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

    let mut map = HashMap::new();

    for elem in vector.iter() {
        let count = map.entry(elem).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    let mode = get_mode_key(&map);
    println!("mode is {}", mode.unwrap());
}

fn get_mode_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}
