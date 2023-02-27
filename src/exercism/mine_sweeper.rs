// Mine Sweeper

// Instructions
// Add the mine counts to a completed Minesweeper board.

// Minesweeper is a popular game where the user has to find the mines using numeric hints that indicate how many mines are directly adjacent (horizontally, vertically, diagonally) to a square.

// In this exercise you have to create some code that counts the number of mines adjacent to a given empty square and replaces that square with the count.

// The board is a rectangle composed of blank space (' ') characters. A mine is represented by an asterisk ('*') character.

// If a given space has no adjacent mines at all, leave that square blank.

pub fn count_mines(s: &str) -> String {
	let bytes = s.as_bytes();
	let mut m = 0;
	let mut n = 0;
	for i in 0..bytes.len() {
		if bytes[i] == b'\n' {
			m = i;
			break;
		}
	}
	if (bytes.len() + 1) % m == 0 {
		n = (bytes.len() + 1) / m;
	} else {
		n = bytes.len() / m;
	}
	let mut res = String::new();
	for i in 0isize..n as isize {
		for j in 0isize..m as isize {
			let func = |x: isize, y: isize| {
				if x < 0 || y < 0 || x as usize >= n || y as usize >= m {
					b'.'
				} else {
					bytes[x as usize * (m + 1 /*\n*/) + y as usize]
				}
			};
			let u = func(i, j);
			if u == b'.' {
				let mut mines = 0;
				for k in -1isize..2 {
					for l in -1isize..2 {
						if func(i + k, j + l) == b'*' {
							mines += 1
						}
					}
				}
				if mines > 0 {
					res.push(('0' as u8 + mines as u8) as char);
				} else {
					res.push(u as char);
				}
			} else {
				res.push(u as char);
			}
		}
		res.push('\n');
	}
	
	res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine_sweeper() {
		assert_eq!(count_mines(".*.*.\n..*..\n..*..\n....."), "1*3*1\n13*31\n.2*2.\n.111.\n")
		
    }
}
