// Grade School

// Instructions
// Given students' names along with the grade that they are in, create a roster for the school.


// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.

use std::collections::{BTreeMap, BTreeSet};
#[allow(clippy::new_without_default)]
pub struct School {
	students: BTreeMap<u32, BTreeSet<String>>
}

impl School {
    pub fn new() -> School {
        School {
			students: BTreeMap::new()
		}
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        match self.students.get_mut(&grade) {
			Some(set) => {
				set.insert(student.into());
			},
			_ => {
				let mut set = BTreeSet::new();
				set.insert(student.into());
				self.students.insert(grade, set);
			}
		};
    }

    pub fn grades(&self) -> Vec<u32> {
		self.students.keys().map(|i| *i).collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
		self.students.get(&grade).unwrap_or(&BTreeSet::new()).iter().map(|s| s.clone()).collect()
    }
}
