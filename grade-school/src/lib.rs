// use std::collections::HashMap;
use std::collections::BTreeMap;
// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades
            .entry(grade)
            .or_insert(Vec::new())
            .push(student.to_owned())
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.grades.get(&grade) {
            Some(roster) => {
                let mut res = roster.clone();
                res.sort();
                res
            }
            None => Vec::new(),
        }
    }
}
