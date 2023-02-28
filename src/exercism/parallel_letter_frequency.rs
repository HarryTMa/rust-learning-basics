// Parallel Letter Frequency

// Instructions
// Count the frequency of letters in texts using parallel computation.

// Parallelism is about doing things in parallel that can also be done sequentially. A common example is counting the frequency of letters. Create a function that returns the total frequency of each letter in a list of texts and that employs parallelism.

use std::collections::HashMap;
use std::thread;
use std::sync::{Mutex, Arc};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
	let n = input.len();
	let each = n / worker_count;
	let map = Arc::new(Mutex::new(HashMap::new()));
	
    let clos = move |i, arc: Arc<Mutex<HashMap<char, usize>>>, end, each| {
		let mut m = arc.lock().unwrap();
		for j in (each * i)..end {
			let s: &str = input[j];
			for ch in s.chars() {
				let mut c = ch;
				if (c as u8 >= b'A' && c as u8 <= b'Z') {
					c = (c as u8 - b'A' + b'a') as char; 
				}
				if (c.is_alphabetic())
				{
					if m.contains_key(&c) {
						*m.get_mut(&c).unwrap() += 1;
					} else {
						m.insert(c, 1);
					}
				}
			}
		}
	};
	
	//let mut handles = vec![];
	for i in 0..worker_count {
		let arc = Arc::clone(&map);
		let end = if i != worker_count - 1 {
			each * (i + 1)
		} else {
			n
		};
		let i2 = i;
		let each2 = each;
		
		thread::scope(|s| {
			s.spawn(move || {
				clos(i2, arc, end, each2);
			});
			//handles.push(handle);
		});
	}
	// for handle in handles {
        // handle.join().unwrap();
    // }
	
	let mut hash_map = HashMap::new();
	let x = &*map.lock().unwrap();
	for key in x.keys() {
		hash_map.insert(*key, x[key]);
	}
	hash_map
}


