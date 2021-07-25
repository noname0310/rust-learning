use std::io::Read;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::iter::FromIterator;

struct MyVec<T>(Vec<T>);

impl<T> Deref for MyVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

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
    let mut myvec = MyVec(inputs);
    myvec.push(1);
	println!("{:?}", myvec);
}