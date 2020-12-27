use md5::{Md5, Digest};
use std::{sync::{Arc, atomic::Ordering}, thread::{self}, time::Duration};
use std::sync::atomic::AtomicI32;
///```
///# use aoc2015::day4::*;
///assert_eq!(p1("bgvyzdsv"), 254575);
///```
pub fn p1(input:&str) -> i32 {
    let result = Arc::new(AtomicI32::new(0));
    for i in 0..8 {
        let input_as_string = input.to_string();
        let res_temp = result.clone();
        thread::spawn(move || find_hash_p1(input_as_string, i, res_temp));
    }
    while result.load(Ordering::Relaxed) == 0 {
        thread::sleep(Duration::from_millis(2))
    }
    result.load(Ordering::Relaxed)
}

pub fn find_hash_p1(input: String, starting_value: i32, result: Arc<AtomicI32>){
    let mut i : i32 = starting_value;
    while result.load(Ordering::Relaxed) == 0 {
        i += 8;
        let output= Md5::digest(format!("{}{}", input, i).as_bytes());
        if output[0] == 0 && output[1] == 0 && output[2] <= 0x0F {
            result.store(i, Ordering::Relaxed);
            break;
        }
    }
}

///```
///# use aoc2015::day4::*;
///assert_eq!(p2("bgvyzdsv"), 1038736);
///```
pub fn p2(input:&str) -> i32 {
    let result = Arc::new(AtomicI32::new(0));
    for i in 0..16 {
        let input_as_string = input.to_string();
        let res_temp = result.clone();
        thread::spawn(move || find_hash_p2(input_as_string, i, res_temp));
    }
    while result.load(Ordering::Relaxed) == 0 {
        thread::sleep(Duration::from_millis(5))
    }
    result.load(Ordering::Relaxed)
}

pub fn find_hash_p2(input: String, starting_value: i32, result: Arc<AtomicI32>){
    let mut i : i32 = starting_value;
    while result.load(Ordering::Relaxed) == 0 {
        i += 16;
        let output= Md5::digest(format!("{}{}", input, i).as_bytes());
        if output[0] == 0 && output[1] == 0 && output[2] == 0 {
            result.store(i, Ordering::Relaxed);
            break;
        }
    }
}