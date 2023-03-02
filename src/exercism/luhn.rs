// Luhn

// Instructions
// Given a number determine whether or not it is valid per the Luhn formula.

// The Luhn algorithm is a simple checksum formula used to validate a variety of identification numbers, such as credit card numbers and Canadian Social Insurance Numbers.

// The task is to check if a given string is valid.

// Validating a Number
// Strings of length 1 or less are not valid. Spaces are allowed in the input, but they should be stripped before checking. All other non-digit characters are disallowed.

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
	let mut vec = Vec::new();
	for ch in code.chars() {
		if ch == ' ' {
			continue;
		}
		if !ch.is_ascii_digit() {
			return false;
		}
		vec.push(ch as u8 - b'0');
	}
	let n = vec.len();
	if n <= 1 {
		return false;
	}
	let mut sum = 0;
	for i in 0..n {
		let j = n - 1 - i;
		let mut u = vec[j];
		if i % 2 == 1 {
			u *= 2;
			if (u > 9) {
				u -= 9;
			}
		}
		sum += u;
	}
	sum % 10 == 0
}
