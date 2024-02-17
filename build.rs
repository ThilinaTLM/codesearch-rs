use std::process::Command;
use std::env;

fn main() {
    if env::var("PROFILE").unwrap() == "release" {
        println!("Running npm run dist for release...");
        Command::new("npm")
            .args(&["run", "build"])
            .current_dir("./web")
            .status()
            .unwrap();
    } else if env::var("PROFILE").unwrap() == "release" {
        println!("Running npm run dist for release...");
        Command::new("mkdir")
            .args(&["-p", "web/dist"])
            .status()
            .unwrap();
    } else {
        panic!("Unknown PROFILE: {}", env::var("PROFILE").unwrap());
    }
}
