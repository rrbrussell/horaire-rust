use std::process::Command;
fn main() {
    println!("Spawning a thread to log something process");
    let mut output = Command::new("logger")
        .arg("I logged something.")
        .spawn()
        .expect("Find the logger command.");
    match output.wait() {
        Err(e) => {
            println!("Logger failed with {e}")
        }
        Ok(out) => {
            println!("Logger exited with {out}")
        }
    }
}
