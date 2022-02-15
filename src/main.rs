extern crate rand;
use std::{thread, time};

fn census(_world: [[u8; 75]; 75]) -> u16 {
    let mut count = 0;

    for i in 0..74 {
        for j in 0..74 {
            if _world[i][j] == 1 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    println!("Hello, world!");
}
