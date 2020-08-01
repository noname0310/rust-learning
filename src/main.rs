use std::io;

#[inline(always)]
fn user_inputs<F: std::str::FromStr,R: io::Read>(r: &mut R) -> Vec<F> {

	let mut text = String::new();
	r.read_to_string(&mut text).expect("Failed to read");
	text
		.split_whitespace()
		.flat_map(str::parse)
		.take(2)
		.collect()
}

fn main() {
	let inputs: [i32; 2] = {
		let mut stdin = io::stdin();
		let inputs = user_inputs::<i32, io::Stdin>(&mut stdin);
		[inputs[0], inputs[1]]
	};

	if inputs[0] != inputs[1] {
		println!("2");
	}
	else {
		let input = inputs[0];
		let mut i = 2;
		while i * i <= input {
			if (input % i) == 0 {
				println!("{}", i);
				return;
			}
			i += 1;
		}
		println!("{}", input);
	}
}