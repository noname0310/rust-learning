use std::io;
use std::vec::Vec;

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

	let mut s: Vec<String> = Vec::new();
	hanoi(input, 1, 3, 2, &mut s);
	println!("{}", s.len());
	for elem in s {
		println!("{}", elem);
	}
}

fn hanoi (n: i32, from: i32, target: i32, other: i32, vec: &mut Vec<String>) {
	if n == 1 {
		vec.push(format!("{} {}", from, target));
		return;
	}
	hanoi(n - 1, from, other, target, vec);
	vec.push(format!("{} {}", from, target));
	hanoi(n - 1, other, target, from, vec);
}