use fern_sim::{run_simulate, Fern};

fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001,
    };
    run_simulate(&mut fern, 1000);
    println!("final fern size: {}", fern.size);
}
