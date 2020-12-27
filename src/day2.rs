///```
///# use aoc2015::day2::*;
///# use aoc2015::helper::*;
///assert_eq!(p1(read_to_string("inputs/day2.txt")), 1586300);
///```
pub fn p1(input:String) -> i32 {
    input
    .lines()
    .map(|line| 
        line
        .split("x")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
    )
    .map(|x| calculate_area(x[0], x[1], x[2]))
    .sum()
}

fn calculate_area(l : i32, w : i32, h : i32) -> i32 {
    let side1 = l * w;
    let side2 = w * h;
    let side3 = h * l;
    (side1 + side2 + side3) * 2 + side1.min(side2).min(side3)
}

///Get the first time we reach a negative floor
///```
///# use aoc2015::day2::*;
///# use aoc2015::helper::*;
///assert_eq!(p2(read_to_string("inputs/day2.txt")), 3737498);
///```
pub fn p2(input:String) -> i32 {
    input
    .lines()
    .map(|line| 
        line
        .split("x")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
    )
    .map(|x| calculate_ribbon_length(x[0], x[1], x[2]))
    .sum()
}

fn calculate_ribbon_length(l : i32, w : i32, h : i32) -> i32 {
    let ribbon_wrap = if l >= w && l >= h {2 * (w + h)} 
                   else if w >= l && w >= h {2 * (l + h)}
                   else {2 * (l + w)};
    ribbon_wrap + (l * w * h)
}