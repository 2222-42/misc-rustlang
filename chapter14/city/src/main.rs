use std::thread;

#[derive(Debug)]
struct City {
    name: String,
    population: i64,
    country: String,
    cost: i64,
    monsster_attack_risk: f64,
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

    fn new(
        name: String,
        population: i64,
        country: String,
        cost: i64,
        monsster_attack_risk: f64,
    ) -> City {
        City {
            name,
            population,
            country,
            cost,
            monsster_attack_risk,
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

fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
where
    F: Fn(&City) -> bool, // クロージャを引数に撮れるようにしたいから。
{
    let mut count = 0;

    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }

    count
}

fn has_monster_attack(city: &City) -> bool {
    city.monsster_attack_risk > 0.0
}

fn main() {
    let mut cities = vec![
        City::new(
            "Tokyo".to_string(),
            13_822_000,
            "Japan".to_string(),
            500_000,
            0.5,
        ),
        City::new(
            "Shanghai".to_string(),
            3_904_000,
            "China".to_string(),
            1_000_000,
            0.2,
        ),
        City::new(
            "Beijing".to_string(),
            21_333_332,
            "China".to_string(),
            2_000_000,
            0.0,
        ),
    ];

    sort_by_statistic(&mut cities, Statistic::Population);

    println!("cities: {:?}", cities);

    let stat = Statistic::Cost;

    let n = count_selected_cities(&cities, has_monster_attack);
    println!("The number of cities with monster attack: {}", n);

    let boundary = 1_000_000;
    let m = count_selected_cities(&cities, |city| city.population > boundary); // city.population > 1_000_000だと通るのはなぜ？？？
                                                                               // 推測になるが、クロージャ独自の型であっても、一定の条件を満たせば型を暗黙のうちに関数の型(fn型)に変換してくれるのだろう。
    println!("The number of cities with population > 1,000,000: {}", m);

    let result = start_sorting_thread(cities, stat).join().unwrap();
    println!("cities: {:?}", result); // citiesにはCopy Traitを実装していないので、moveしており、citiesの変数からアクセスできない。
                                      // citiesを以降で使いたいなら、cloneをして別の変数に格納しましょう。
    println!("stat: {:?}", stat); // statにはCopy Traitを実装しているので、statの変数からアクセスできる。
}
