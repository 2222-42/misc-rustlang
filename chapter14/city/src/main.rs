use std::thread;

#[derive(Debug)]
struct City {
    name: String,
    population: i64,
    country: String,
    cost: i64,
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population); // 無名関数を使った方がシンプルなので。
                                                 // -city.populationで降順にソート
}

#[derive(Copy, Clone, Debug)] // get_statisticに渡すところで以下のエラーが出たので。p. 301の説明と矛盾が起きそう？
                              // cannot move out of `stat`, a captured variable in an `FnMut` closure
                              // move occurs because `stat` has type `Statistic`, which does not implement the `Copy` trait
enum Statistic {
    Population,
    Cost,
}

impl City {
    // Stasticの型の指定は教科書にはなかったが、Statisticと&Statistic、どちらがよかったのだろうか。
    fn get_statistic(&self, stat: Statistic) -> i64 {
        match stat {
            Statistic::Population => self.population,
            Statistic::Cost => self.cost,
        }
    }
}

fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) {
    cities.sort_by_key(|city| -city.get_statistic(stat)); // 借用するクロージャらしい。
}

fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic) -> thread::JoinHandle<Vec<City>> {
    let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) }; // statの所有権を取得

    thread::spawn(move || {
        // citiesとkey_fnの所有権を取得
        cities.sort_by_key(key_fn);
        cities
    })
}

fn main() {
    let mut cities = vec![
        City {
            name: "Tokyo".to_string(),
            population: 13_822_000,
            country: "Japan".to_string(),
            cost: 500_000,
        },
        City {
            name: "Shanghai".to_string(),
            population: 3_904_000,
            country: "China".to_string(),
            cost: 1_000_000,
        },
        City {
            name: "Beijing".to_string(),
            population: 21_333_332,
            country: "China".to_string(),
            cost: 2_000_000,
        },
    ];

    sort_by_statistic(&mut cities, Statistic::Population);

    println!("cities: {:?}", cities);

    let stat = Statistic::Cost;

    let result = start_sorting_thread(cities, stat).join().unwrap();
    println!("cities: {:?}", result); // citiesにはCopy Traitを実装していないので、moveしており、citiesの変数からアクセスできない。
                                      // citiesを以降で使いたいなら、cloneをして別の変数に格納しましょう。
    println!("stat: {:?}", stat); // statにはCopy Traitを実装しているので、statの変数からアクセスできる。
}
