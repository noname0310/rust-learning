use std::io::Read;
use std::str::FromStr;
use std::iter::FromIterator;

fn user_input<I, R, T, const N: usize>(mut r: R) -> I
where
    I: FromIterator<T>,
    T: FromStr,
    R: Read,
{
    let mut text = String::new();
    r.read_to_string(&mut text).unwrap();

    text.split_whitespace()
        .flat_map(|s|s.parse())
		.take(N)
        .collect()
}

fn main() {
	let stdin = std::io::stdin();
	let inputs: Vec<i32> = user_input::<_, _, _, 10>(stdin);
	println!("{:?}", inputs);
}