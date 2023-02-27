// Sublist

// Instructions
// Given two lists determine if the first list is contained within the second list, if the second list is contained within the first list, if both lists are contained within each other or if none of these are true.

// Specifically, a list A is a sublist of list B if by dropping 0 or more elements from the front of B and 0 or more elements from the back of B you get a list that's completely equal to A.

// Checks if A is sublist of B
pub fn is_sublist<T: PartialEq>(A: &Vec<T>, B: &Vec<T>) -> bool {
	let la = A.len() as isize;
	let lb = B.len() as isize;
	for i in 0..lb - la + 1 {
		let mut res = true;
		for j in 0..la {
			if &A[j as usize] != &B[(i + j) as usize] {
				res = false;
				break;
			}
		}
		if res {
			return true;
		}
	}
	false
}

pub fn compare_lists<T: PartialEq>(A: &Vec<T>, B: &Vec<T>) -> &'static str {
	let b1 = is_sublist(A, B);
	let b2 = is_sublist(B, A);
	if b1 {
		if b2 {
			"A is equal to B"
		} else {
			"A is a sublist of B"
		}
	} else {
		if b2 {
			"A is a superlist of B"
		} else {
			"A is not a superlist of, sublist of or equal to B"
		}
	}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sublists() {
		assert_eq!(compare_lists(&vec![1, 2, 3], &vec![1, 2, 3, 4, 5]), "A is a sublist of B");
		assert_eq!(compare_lists(&vec![3, 4, 5], &vec![1, 2, 3, 4, 5]), "A is a sublist of B");
		assert_eq!(compare_lists(&vec![3, 4], &vec![1, 2, 3, 4, 5]), "A is a sublist of B");
		assert_eq!(compare_lists(&vec![1, 2, 3], &vec![1, 2, 3]), "A is equal to B");
		assert_eq!(compare_lists(&vec![1, 2, 3, 4, 5], &vec![2, 3, 4]), "A is a superlist of B");
		assert_eq!(compare_lists(&vec![1, 2, 4], &vec![1, 2, 3, 4, 5]), "A is not a superlist of, sublist of or equal to B");
		
    }
}
