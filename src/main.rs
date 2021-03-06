use shared::*;
use tutorial::*;

mod shared;
mod tutorial;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Call with the number of the tutorial, e.g. `1_1_2` for _1_2_hello_window_clear.rs");
        std::process::exit(1);
    }
    let tutorial_id = &args[1];

    match tutorial_id.as_str() {
        #[cfg(feature = "chapter-1")]
        "1_1_1" => main_1_1_1(),
        #[cfg(feature = "chapter-1")]
        "1_1_2" => main_1_1_2(),
        #[cfg(feature = "chapter-1")]
        "1_2_1" => main_1_2_1(),
        #[cfg(feature = "chapter-1")]
        "1_2_2" => main_1_2_2(),
        #[cfg(feature = "chapter-1")]
        "1_2_3" => main_1_2_3(),
        #[cfg(feature = "chapter-1")]
        "1_2_4" => main_1_2_4(),
        #[cfg(feature = "chapter-1")]
        "1_2_5" => main_1_2_5(),
        #[cfg(feature = "chapter-1")]
        "1_3_1" => main_1_3_1(),
        #[cfg(feature = "chapter-1")]
        "1_3_2" => main_1_3_2(),
        #[cfg(feature = "chapter-1")]
        "1_3_3" => main_1_3_3(),
        #[cfg(feature = "chapter-1")]
        "1_4_1" => main_1_4_1(),
        #[cfg(feature = "chapter-1")]
        "1_4_2" => main_1_4_2(),
        #[cfg(feature = "chapter-1")]
        "1_5_1" => main_1_5_1(),
        #[cfg(feature = "chapter-1")]
        "1_6_1" => main_1_6_1(),
        #[cfg(feature = "chapter-1")]
        "1_6_2" => main_1_6_2(),
        #[cfg(feature = "chapter-1")]
        "1_6_3" => main_1_6_3(),
        #[cfg(feature = "chapter-1")]
        "1_7_1" => main_1_7_1(),
        #[cfg(feature = "chapter-1")]
        "1_7_2" => main_1_7_2(),
        #[cfg(feature = "chapter-1")]
        "1_7_3" => main_1_7_3(),
        #[cfg(feature = "chapter-1")]
        "1_7_4" => main_1_7_4(),

        #[cfg(feature = "chapter-2")]
        "2_1" => main_2_1(),
        #[cfg(feature = "chapter-2")]
        "2_2_1" => main_2_2_1(),
        #[cfg(feature = "chapter-2")]
        "2_2_2" => main_2_2_2(),
        #[cfg(feature = "chapter-2")]
        "2_3_1" => main_2_3_1(),
        #[cfg(feature = "chapter-2")]
        "2_4_1" => main_2_4_1(),
        #[cfg(feature = "chapter-2")]
        "2_4_2" => main_2_4_2(),
        #[cfg(feature = "chapter-2")]
        "2_5_1" => main_2_5_1(),
        #[cfg(feature = "chapter-2")]
        "2_5_2" => main_2_5_2(),
        #[cfg(feature = "chapter-2")]
        "2_5_3" => main_2_5_3(),
        #[cfg(feature = "chapter-2")]
        "2_5_4" => main_2_5_4(),
        #[cfg(feature = "chapter-2")]
        "2_6" => main_2_6(),

        #[cfg(feature = "chapter-3")]
        "3_1" => main_3_1(),

        #[cfg(feature = "chapter-4")]
        "4_1_1" => main_4_1_1(),
        "4_1_2" => main_4_1_2(),
        "4_2" => main_4_2(),
        "4_3_1" => main_4_3_1(),

        _ => println!("unknown tutorial id"),
    }
}
