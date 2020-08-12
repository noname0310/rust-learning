use std::io;
use std::vec::Vec;
use std::collections::BinaryHeap;

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
	let input = {
		let inputs = user_inputs::<i32>(2);
		(inputs[0], inputs[1])
	};
	let v = input.0 + 1;
	let _e = input.1;

	let input = {
		let inputs = user_inputs::<i32>(1);
		inputs[0]
	};
	let start = input;
	
	let max = 20001;
	let mut adj: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0); 3]; max];

	for _i in 0.._e {
		let input = {
			let inputs = user_inputs::<i32>(3);
			(inputs[0], inputs[1], inputs[2])
		};

		let x = input.0;
		let y = input.1;
		let z = input.2;

		adj[x as usize].push((y, z));
	}

	let vec = dijkstra(start as usize, v as usize, adj);
	for i in 1..v {
		if vec[i as usize] == std::i32::MAX {
			println!("INF")
		} else {
			println!("{}", vec[i as usize]);
		}
	}
}

fn dijkstra(src: usize, v: usize, adj: Vec<Vec<(i32, i32)>>) -> Vec<i32> {
	let mut dist = vec![std::i32::MAX; v];
	dist[src] = 0;

	let mut pq = BinaryHeap::new();
	pq.push((0, src));

	while !pq.is_empty() {
		let cost = -(pq.peek().unwrap().0);
		let here = pq.peek().unwrap().1;

		pq.pop().unwrap();

		if dist[here] < cost {
			continue;
		}

		for i in 0..(adj[here].len()) {
			let there = adj[here][i].0 as usize;
			let next = cost + adj[here][i].1;

			if dist[there] > next {
				dist[there] = next;
				pq.push((-next, there));
			}
		}
	}

	return dist;
}