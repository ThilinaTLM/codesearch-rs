use std::process::Command;
use std::env;

fn main() {
    if env::var("PROFILE").unwrap() == "release" {
        println!("Running npm run dist for release...");
        // Navigate to the web directory and run npm run dist
        Command::new("npm")
            .args(&["run", "dist"])
            .current_dir("./web")
            .status()
            .unwrap();
    }
}
