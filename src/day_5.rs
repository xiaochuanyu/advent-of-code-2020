use crate::util;
use std::io::BufRead;

pub fn part_1() {
    let file = util::read_input();
    let mut max_id = 0;
    for line in file.lines().map(|result| result.unwrap()) {
        let mut row: i32 = 0;
        let mut col: i32 = 0;
        for (i, c) in line.chars().enumerate() {
            let bit;
            if i < 7 { // row
                if c == 'F' {
                    bit = 0;
                } else {
                    bit = 1;
                }
                row = (row << 1) | bit;
            } else { // col
                if c == 'L' {
                    bit = 0;
                } else {
                    bit = 1;
                }
                col = (col << 1) | bit;
            }
        }
        let row_id = row * 8 + col;
        if max_id < row_id {
            max_id = row_id;
        }
    }
    println!("{}", max_id);
}

pub fn part_2() {
    let file = util::read_input();
    let mut max_row = 0;
    let mut min_row = i32::MAX;
    let mut seat_ids: [bool; 891] = [false; 891];
    for line in file.lines().map(|result| result.unwrap()) {
        let mut row: i32 = 0;
        let mut col: i32 = 0;
        for (i, c) in line.chars().enumerate() {
            let bit;
            if i < 7 { // row
                if c == 'F' {
                    bit = 0;
                } else {
                    bit = 1;
                }
                row = (row << 1) | bit;
            } else { // col
                if c == 'L' {
                    bit = 0;
                } else {
                    bit = 1;
                }
                col = (col << 1) | bit;
            }
        }
        let seat_id = row * 8 + col;
        seat_ids[seat_id as usize] = true;
        if max_row < row {
            max_row = row;
        }
        if min_row > row {
            min_row = row;
        }
    }
    for (seat_id, is_taken) in seat_ids.iter().enumerate() {
        // println!("{} {} {}", seat_id, seat_id / 8, row);
        let row = seat_id as i32 / 8;
        if !is_taken && row > min_row && row < max_row {
            println!("{}", seat_id);
            return;
        }
    }
}

