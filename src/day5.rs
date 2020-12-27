use std::collections::HashSet;

///```
///# use aoc2015::day5::*;
///# use aoc2015::helper::*;
///assert_eq!(p1(read_to_string("inputs/day5.txt")), 258);
///```
pub fn p1(input:String) -> i32 {
    input
    .lines()
    .filter(|x| is_nice_p1(x.as_bytes()))
    .collect::<Vec<&str>>()
    .len() as i32
}

fn is_nice_p1(input: &[u8]) -> bool {
    let mut vowel_count = 0;
    let mut has_double = false;
    let mut naughty_combos = false;

    for i in 0..input.len() {
        if [b'a', b'i', b'u', b'e', b'o'].contains(&input[i]) {
            vowel_count += 1
        }
        if i != input.len() - 1 && input[i] == input[i + 1] {
            has_double = true
        }
        if i != input.len() - 1 && [[b'a', b'b'], [b'c', b'd'], [b'p', b'q'], [b'x', b'y']].contains(&[input[i], input[i + 1]]) {
            naughty_combos =  true;
            break;
        }
    }
    return vowel_count >= 3 && has_double && !naughty_combos
}
///```
///# use aoc2015::day5::*;
///# use aoc2015::helper::*;
///assert_eq!(p2(read_to_string("inputs/day5.txt")), 53);
///```
pub fn p2(input:String) -> i32 {
    input
    .lines()
    .filter(|x| is_nice_p2(x.as_bytes()))
    .collect::<Vec<&str>>()
    .len() as i32
}

fn is_nice_p2(input: &[u8]) -> bool {
    let mut prev_c = 0 as u8;
    let mut prev_prev_c = 0 as u8;
    let mut hashset : HashSet<[u8; 2]> = HashSet::new();
    let mut repeats_one_apart = false;
    let mut two_letter_combo = false;
    for c in input {
        if &prev_prev_c == c {
            repeats_one_apart = true;
        }
        if hashset.contains(&[prev_c, *c])  {
            two_letter_combo = true;
        }
        hashset.insert([prev_prev_c, prev_c]);
        prev_prev_c = prev_c;
        prev_c = *c;
    }
    repeats_one_apart && two_letter_combo
}