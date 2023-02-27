// Part 3
// This part includes practice problems for learning Rust on Test

// Write a second test that tests that we get the result
// we expect to get when we call `times_two` with a negative number.

pub fn times_two(num: i32) -> i32 {
    num * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_twice_of_positive_numbers() {
        assert_eq!(times_two(4), 8);
    }

    #[test]
    fn returns_twice_of_negative_numbers() {
        // TODO replace unimplemented!() with an assert for `times_two(-4)`
		assert_eq!(times_two(-4), -8);
    }
}