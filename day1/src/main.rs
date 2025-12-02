// we have a dial, with numbers 0 to 99
// going back from 0, 1 step will take you to 99
// going from 90, 15 steps will take you to 4
// the dial starts at 50
// each line of the input has L or R as the first char, with number of moves from 0 to 999
// we need to keep track of the amount of times that we reach 0 whilst turning the dial.

use std::fs;

fn move_dial(mut dial: i32, direction: char, mut steps: i32) -> i32 {
    // if a number of steps is larger than 99 in either direction
    // we can ignore these full rotations (100 steps for 1 full rotation)
    if steps > 99 {
        steps = steps % 100
    }
    // now match direction and go forwards and backwards on the dial
    match direction {
        'L' => {
            println!("left | steps: {} dial: {}", steps, dial);
            // check if we are crossing the 0 mark on the dial
            if dial - steps < 0 {
                dial -= steps;
                dial += 100;
            } else {
                dial -= steps;
            }
        }
        'R' => {
            println!("right | steps: {} dial: {}", steps, dial);
            // check if we are crossing the 0 mark on the dial
            if dial + steps > 99 {
                dial += steps;
                dial -= 100; // we crossed it!
            } else {
                dial += steps;
            }
        }
        _ => {}
    }
    return dial;
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // dial starts at 50
    let mut dial = 50;

    // count the number of times we hit 0
    let mut zero_hits = 0;

    // iterate lines in file
    contents.split('\n').for_each(|line| {
        // iterate line chars
        let mut line_chars = line.chars();

        // direction is L or R first char of line
        let direction = line_chars.nth(0).unwrap();

        // steps is the rest of the chars from 0 to 999
        let steps: i32 = line_chars.as_str().parse().unwrap();

        // move the dial
        dial = move_dial(dial, direction, steps);
        if dial == 0 {
            zero_hits += 1;
        }
    });

    println!("password is: {}", zero_hits)
}
