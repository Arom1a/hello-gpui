use std::env;

mod gpui_example;
mod gpui_kit_example;

#[cfg(not(target_family = "wasm"))]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1] == "kit" {
            gpui_kit_example::run_example();
        }
        if args[1] == "official" {
            gpui_example::run_example();
        }
    } else {
        println!(
            "Please provide the example you want to run, e.g. `{} kit`",
            args[0]
        )
    }
}
