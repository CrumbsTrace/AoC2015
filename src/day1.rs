///Get the final floor reached
///```
///# use aoc2015::day1::*;
///# use aoc2015::helper::*;
///assert_eq!(p1(read_to_string("inputs/day1.txt")), 74);
///```
pub fn p1(input:String) -> i32 {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == '(' {1} else {-1})
        .sum::<i32>()
}

///Get the first time we reach a negative floor
///```
///# use aoc2015::day1::*;
///# use aoc2015::helper::*;
///assert_eq!(p2(read_to_string("inputs/day1.txt")), 1795);
///```
pub fn p2(input:String) -> i32 {
    let line = input.lines().next().unwrap();
    let line_mapped : Vec<i32> = line.chars().map(|c| if c == '(' {1} else {-1}).collect();
    let mut sum = 0;
    let mut i = 0;
    for elem in line_mapped.iter() {
        sum += elem;
        i += 1;
        if sum < 0 {
            return i
        }
    }

    0
}
