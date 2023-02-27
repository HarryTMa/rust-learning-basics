// Part 4
// This part includes practice problems for learning Rust on Modules and Macros.



// Write a macro that passes the test!


macro_rules! my_macro {
	($x:expr) => {
		format!("Hello {}", $x)
	}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}