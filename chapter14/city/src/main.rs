#[derive(Debug)]
struct City {
    name: String,
    population: i64,
    country: String,
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population); // 無名関数を使った方がシンプルなので。
                                                 // -city.populationで降順にソート
}

fn main() {
    let mut cities = vec![
        City {
            name: "Tokyo".to_string(),
            population: 13_822_000,
            country: "Japan".to_string(),
        },
        City {
            name: "Shanghai".to_string(),
            population: 3_904_000,
            country: "China".to_string(),
        },
        City {
            name: "Beijing".to_string(),
            population: 21_33_332,
            country: "China".to_string(),
        },
    ];

    sort_cities(&mut cities);

    println!("{:?}", cities);
}
