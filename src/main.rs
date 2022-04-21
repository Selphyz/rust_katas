use std::collections::hash_map::Values;
use std::iter::Rev;
use std::str::Split;

mod test;

fn two_sort(arr: &[&str]) -> String {
    // your code here
    arr.iter().min().unwrap().chars().map(|c| c.to_string()).collect::<Vec<_>>().join("***")
}
fn name_shuffler(s: &str) -> String {
    s.rsplit(" ").collect::<Vec<&str>>().join(" ")
    // s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}
fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    i32::abs(a.iter().product::<i32>() - b.iter().product::<i32>())
}
fn main() {
    println!("Hello, world!");
}
