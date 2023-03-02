// Poker

// Instructions
// Pick the best hand(s) from a list of poker hands.

// See wikipedia for an overview of poker hands.

// Ranking a list of poker hands can be considered a sorting problem.
// Rust provides the sort method for Vec<T> where T: Ord.
// Ord types form a total order: exactly one of a < b, a == b, or a > b must be true.
// Poker hands do not conform to a total order: it is possible for two hands to be non-equal but have equal sort order. Example: "3S 4S 5D 6H JH", "3H 4H 5C 6C JD".
// Rust provides the PartialOrd trait to handle the case of sortable things which do not have a total order. However, it doesn't provide a standard sort method for Vec<T> where T: PartialOrd. The standard idiom to sort a vector in this case is your_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::{Less|Equal|Greater}));, depending on your needs.
// You might consider implementing a type representing a poker hand which implements PartialOrd.

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

use std::cmp::Ordering;

pub fn poker_parse(x: &[u8]) -> Vec<(u32, u8)> {
	let mut result = Vec::new();
	for i in 0..5 {
		let ord = {
			let u = x[3 * i];
			if u >= b'2' && u <= b'9' {
				(u - b'2' + 2) as u32
			} else {
				match u {
					b'T' => 10,
					b'J' => 11,
					b'Q' => 12,
					b'K' => 13,
					b'A' => 14,
					_ => 0
				}
			}
		};
		let suit = x[3 * i + 1];
		result.push((ord, suit));
	}
	result
}

pub fn poker_categorize(x: &Vec<(u32, u8)>)-> (u8, u32) {
	// I guess there's no possibility that it's `Five of a Kind`
	// So the rank is:
	// Straight Flush:		10
	// Four of a Kind:		9
	// Full house:			8
	// Flush:				7
	// Straight:			6
	// Three of a kind:		5
	// Two pair:			4
	// One pair:			3
	// High card:			2
	
	let mut straight = true;
	let x0 = x[0].0;
	for i in 1..5 {
		if x[i].0 != x0 + i as u32 {
			straight = false;
			break;
		}
	}
	// However, we should consider "A2345"
	if x[0].0 == 2 && x[1].0 == 3 && x[2].0 == 4 && x[3].0 == 5 && x[4].0 == 14 {
		straight = true;
	}
	
	let mut flush = true;
	let y0 = x[0].1;
	for i in 1..5 {
		if x[i].1 != y0 {
			flush = false;
			break;
		}
	}
	
	if straight && flush {
		if x[0].0 == 2 && x[4].0 == 14 {
			return (10, 5);
		}
		return (10, (x0 + 4) as u32);
	}
	if straight {
		if x[0].0 == 2 && x[4].0 == 14 {
			return (6, 5);
		}
		return (6, (x0 + 4) as u32);
	}
	if flush {
		let mut val = 0;
		for i in 0..5 {
			val |= x[i].0 << (i << 2);
		}
		return (7, val);
	}
	
	if x[1].0 == x[3].0 {
		// the only way to be `four of a kind`
		if x[0].0 == x[1].0 {
			return (9, (x[0].0 << 4) | x[4].0 as u32);
		}
		if x[3].0 == x[4].0 {
			return (9, (x[4].0 << 4) | x[0].0 as u32);
		}
		// then it's `three of a kind`
		// notice, it can't be `full house` now
		return (5, (x[1].0 << 8) | (x[4].0 << 4) | x[0].0 as u32);
	}
	
	if x[0].0 == x[2].0 {
		// may be `three of a kind` or `full house`
		if x[3].0 == x[4].0 {
			return (8, (x[2].0 << 4) | x[3].0 as u32);
		}
		return (5, (x[2].0 << 8) | (x[4].0 << 4) | x[3].0 as u32);
	}
	if x[2].0 == x[4].0 {
		if x[0].0 == x[1].0 {
			return (8, (x[2].0 << 4) | x[1].0 as u32);
		}
		return (5, (x[2].0 << 8) | (x[1].0 << 4) | x[0].0 as u32);
	}
	
	// now check pairs
	if x[0].0 == x[1].0 {
		if x[2].0 == x[3].0 {
			return (4, (x[3].0 << 8) | (x[1].0 << 4) | x[4].0 as u32);
		}
		if x[4].0 == x[3].0 {
			return (4, (x[3].0 << 8) | (x[1].0 << 4) | x[2].0 as u32);
		}
		return (3, (x[1].0 << 12) | (x[4].0 << 8) | (x[3].0 << 4) | x[2].0 as u32);
	}
	
	// x[0].0 != x[1].0
	if x[1].0 == x[2].0 {
		if x[3].0 == x[4].0 {
			return (4, (x[3].0 << 8) | (x[1].0 << 4) | x[0].0 as u32);
		}
		return (3, (x[1].0 << 12) | (x[4].0 << 8) | (x[3].0 << 4) | x[0].0 as u32);
	}
	
	if x[2].0 == x[3].0 {
		return (3, (x[2].0 << 12) | (x[4].0 << 8) | (x[1].0 << 4) | x[0].0 as u32);
	}
	
	if x[3].0 == x[4].0 {
		return (3, (x[3].0 << 12) | (x[2].0 << 8) | (x[1].0 << 4) | x[0].0 as u32);
	}
	
	let mut val = 0;
	for i in 0..5 {
		val |= x[i].0 << (i << 2);
	}
	return (2, val);
}



pub fn poker_cmp(a: &&str, b: &&str) -> Ordering {
	let mut va = poker_parse(a.replace("10", "T").as_bytes());
	let mut vb = poker_parse(b.replace("10", "T").as_bytes());
	let method = |a: &(u32, u8), b: &(u32, u8)| {
		if a.0 != b.0 {
			a.0.cmp(&b.0)
		} else {
			a.1.cmp(&b.1)
		}
	};
	va.sort_by(method);
	vb.sort_by(method);
	let ca = poker_categorize(&va);
	let cb = poker_categorize(&vb);
	if ca.0 == cb.0 {
		ca.1.cmp(&cb.1)
	} else {
		ca.0.cmp(&cb.0)
	}
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let n = hands.len();
	let mut vec = Vec::new();
	for i in 0..n {
		vec.push(hands[i]);
	}
	vec.sort_by(poker_cmp);
	let mut result = Vec::new();
	result.push(vec[n - 1]);
	for i in 0..n - 1 {
		let j = n - 1 - i;
		if poker_cmp(&vec[j], &vec[j - 1]) == Ordering::Equal {
			result.push(vec[j - 1]);
		} else {
			break;
		}
	}
	result
}


#[test]
pub fn poker_test() {
	assert_eq!(poker_cmp(&"3C 4D 5S 6H JD", &"3D 4C 5D 6D JS"), Ordering::Equal);
	assert_eq!(poker_cmp(&"4S 5H 5S 5D 5C", &"7S 8S 9S 6S 10S"), Ordering::Less);
	
}