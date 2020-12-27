use std::collections::HashSet;

///```
///# use aoc2015::day3::*;
///# use aoc2015::helper::*;
///assert_eq!(p1(read_to_string("inputs/day3.txt")), 2081);
///```
pub fn p1(input:String) -> i32 {
    let chars = input.lines().next().unwrap().chars();
    let mut current_pos = (0, 0);
    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert(current_pos);
    for c in chars {
        let dir = char_to_direction_vector(c);
        current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
        visited.insert(current_pos);
    }
    visited.len() as i32
}

fn char_to_direction_vector(c:char) -> (i32, i32) {
    return match c {
        '^' => (0, 1),
        '>' => (1, 0),
        'v' => (0, -1),
        _   => (-1, 0)
    }
}
///Get the first time we reach a negative floor
///```
///# use aoc2015::day3::*;
///# use aoc2015::helper::*;
///assert_eq!(p2(read_to_string("inputs/day3.txt")), 2341);
///```
pub fn p2(input:String) -> i32 {
    let chars = input.lines().next().unwrap().chars();
    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);
    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert(pos1);
    for (i, c) in chars.enumerate() {
        let dir = char_to_direction_vector(c);
        if i % 2 == 0 {
            pos1 = (pos1.0 + dir.0, pos1.1 + dir.1);
            visited.insert(pos1);
        } else {
            pos2 = (pos2.0 + dir.0, pos2.1 + dir.1);
            visited.insert(pos2);
        }
    }
    visited.len() as i32
}