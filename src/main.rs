use std::io;

#[inline(always)]
fn user_inputs<F: std::str::FromStr>() -> Vec<F> {

	let mut text = String::new();
	io::stdin().read_line(&mut text).expect("Failed to read");
	text
		.split_whitespace()
		.flat_map(str::parse)
		.take(1)
		.collect()
}

fn main() {
	let input = {
		let inputs = user_inputs::<i32>();
		inputs[0]
	};

	if input % 3 == 0 {
		print!("{}", input/3);
	} else if input % 2 == 0 {
		print!("{}", input/2);
	} else if input % 3 == 0 {
		print!("{}", input/3);
	}
}