use crate::{ProblemDescription, ProblemObject, ProblemSolver};
// use std::collections::HashMap;

pub fn template(part: usize, _buffer: String) {
    if part == 1 {
        dbg!(World::parse(ProblemDescription::None).part1());
    } else {
        dbg!(World::parse(ProblemDescription::None).part2());
    }
}

#[derive(Debug, PartialEq)]
struct Object {}

impl ProblemObject for Object {
    fn parse(_s: &str) -> Option<Self> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
struct World {}

impl ProblemSolver<Object, usize, String> for World {
    const DAY: usize = 0;
    const DELIMITER: &'static str = "\n";
    fn insert(&mut self, _object: Object) {
        todo!()
    }
    fn default() -> Self {
        World {}
    }
    fn part1(&mut self) -> usize {
        0
    }
    fn part2(&mut self) -> String {
        "done".to_string()
    }
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        const TEST1: &str = "0\n1\n2";
        assert_eq!(
            World::parse(ProblemDescription::TestData(TEST1.to_string())).part1(),
            0
        );
    }
    #[test]
    fn test_part2() {
        const TEST2: &str = "0\n1\n2";
        assert_eq!(
            World::parse(ProblemDescription::TestData(TEST2.to_string())).part2(),
            "done".to_string()
        );
    }
}