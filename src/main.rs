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
		let inputs = user_inputs::<usize>();
		inputs[0]
	};

	let mut array = vec![vec![' '; input]; input];
	array[0][0] = '*'; array[0][1] = '*'; array[0][2] = '*';
	array[1][0] = '*';                    array[1][2] = '*';
	array[2][0] = '*'; array[2][1] = '*'; array[2][2] = '*';

	pattern(&mut array, 3, input);

	for elem in array {
		for elem1 in elem {
			print!("{}", elem1);
		}
		println!();
	}
}

fn pattern(array: &mut Vec<Vec<char>>, width: usize, endcond: usize) {
	if width >= endcond {
		return;
	}
	for i in 0..width {
		for j in width..width * 2 {
			array[i][j] = array[i][j - width];
		}
	}
	for i in 0..width {
		for j in width * 2..width * 3{
			array[i][j] = array[i][j - (width * 2)];
		}
	}

	for i in width..width * 2 {
		for j in 0..width {
			array[i][j] = array[i - width][j];
		}
	}
	for i in width..width * 2 {
		for j in width * 2..width * 3 {
			array[i][j] = array[i - width][j - (width * 2)];
		}
	}
	
	for i in width * 2..width * 3 {
		for j in 0..width {
			array[i][j] = array[i - (width * 2)][j];
		}
	}
	for i in width * 2..width * 3 {
		for j in width..width * 2 {
			array[i][j] = array[i - (width * 2)][j - width];
		}
	}
	for i in width * 2..width * 3 {
		for j in width * 2..width * 3 {
			array[i][j] = array[i - (width * 2)][j - (width * 2)];
		}
	}
	pattern(array, width * 3, endcond);
}