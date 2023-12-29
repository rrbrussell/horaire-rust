use std::process::Command;
use std::thread;
use std::time;

fn main() {
	println!("Spawning a thread to log something on.");
	let thread_join_handle = thread::spawn(move || {
		let test_times = vec![
			time::Duration::new(15, 0),
			time::Duration::new(30, 0),
			time::Duration::new(60, 0),
		];
		for time in test_times {
			thread::sleep(time);
			println!("Spawning a process to log something.");
			let mut output = Command::new("logger")
				.arg("I logged something.")
				.spawn()
				.expect("Find the logger command.");
			match output.wait() {
				Err(e) => {
					println!("Logger failed with {e}")
				},
				Ok(out) => {
					println!("Logger exited with {out}")
				},
			}
		}
	});
	let result = thread_join_handle.join();
	match result {
		Err(_) => {
			println!("The thread did not work.")
		},
		Ok(_) => {
			println!("The thread did work.\nPlease check /var/log/messages.")
		},
	}
}
