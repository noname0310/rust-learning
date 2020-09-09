use std::io;
use std::vec::Vec;

#[inline(always)]
fn user_inputs<F: std::str::FromStr>(val: usize) -> Vec<F> {

	let mut text = String::new();
	io::stdin().read_line(&mut text).expect("Failed to read");
	text
		.split_whitespace()
		.flat_map(str::parse)
		.take(val)
		.collect()
}

fn main() {
	let n = {
		let inputs = user_inputs::<usize>(2);
		inputs[0]
	};
	let mut val = [0; 27];
	let mut stack = Vec::new();
	
	let mut str = String::new();
	let mut a = io::stdin().read_line(&mut str);

	for i in 0..n {
		val[i] = user_inputs::<usize>(1)[0];
	}

	for i in 0..str.len() {
		if str.as_bytes()[i] >= 'A' as u8 && str.as_bytes()[i] <= 'Z' as u8 {
			let index = str.as_bytes()[i] - 'A' as u8;
			stack.push(val[index as usize]);
		} else {
			let temp2 = stack.pop().unwrap();
			let temp1 = stack.pop().unwrap();

			if str.as_bytes()[i] == '*' as u8 {
				stack.push(temp1 * temp2);
			} else if str.as_bytes()[i] == '+' as u8 {
				stack.push(temp1 * temp2);
			} else if str.as_bytes()[i] == '-' as u8 {
				stack.push(temp1 * temp2);
			} else if str.as_bytes()[i] == '/' as u8 {
				stack.push(temp1 * temp2);
			}
		}
	}
}