mod shared;
mod tutorial;

use shared::*;
use tutorial::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Call with the number of the tutorial, e.g. `1_1_2` for _1_2_hello_window_clear.rs");
        std::process::exit(1);
    }
    let tutorial_id = &args[1];

    match tutorial_id.as_str() {
        #[cfg(feature = "chapter-1")] "1_1_1" => main_1_1_1(),

        _ => println!("unknown tutorial id"),
    }
}
