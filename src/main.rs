use std::env;

mod day_one;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("parameters required: day file");
    }

    let day: &str = &args[1];
    let filename: &str = &args[2];

    match day {
        "one" => day_one::run(filename),
        _ => unimplemented!(),
    }
}
