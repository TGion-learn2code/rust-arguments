use std::env;

// main function
fn main() {
    let args: Vec<String> = env::args().collect();

	// Print all arguments on the console
	println!("Arguments are {:?}", args);
}
