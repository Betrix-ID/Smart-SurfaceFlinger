mod duration;
mod help;

use duration::*;
use std::{env, process::exit, thread::sleep, time::Duration};
use help::usage;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
        exit(1);
    }

    match args[1].as_str() {
        "-d" => {
            println!("- Apply duration 120hz");
            surface_flinger120();
        }
        "-L" => {
            println!("- Apply duration 90hz");
            surface_flinger90();
        }
        "-O" => {
            println!("- Apply duration 60hz");
            surface_flinger60();
        }
        "-P" => {
            println!("- Apply duration smart auto SurfaceFlinger");
            monitor_auto();
        }
        "-R" => {
            println!("\nDescription:\n  Resetting SurfaceFlinger to default settings.");
            kill();
        }
        "-h" | "--help" => usage(),
        other => {
            println!("Unknown option: {}", other);
            exit(1);
        }
    }

    sleep(Duration::from_secs(1));
    println!("\n⚠️ This module is protected by copyright and is\n\
              intended for use by regular users only. Any use of\n\
              this module, including its code, design, or features,\n\
              by other developers without written permission from\n\
              the copyright owner is strictly prohibited.\n\
              ________________________________________________(+)\n");
}