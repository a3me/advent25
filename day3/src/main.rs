use std::fs;

fn main() {
    let input_string =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut part1_total_joltage: usize = 0;

    input_string.split("\n").for_each(|bank| {
        let batteries: Vec<char> = bank.chars().collect();

        let mut first_battery_index: usize = 0;
        let mut first_battery_joltage: u32 = batteries[0].to_digit(10).unwrap();

        // find the best battery from the start
        for i in 0..batteries.len() - 1 {
            let battery = batteries[i];
            let battery_joltage: u32 = battery.to_digit(10).unwrap();
            if battery_joltage > first_battery_joltage {
                first_battery_index = i;
                first_battery_joltage = battery_joltage;
            }
        }

        let mut second_battery_index: usize = first_battery_index + 1;
        let mut second_battery_joltage: u32 = batteries[second_battery_index].to_digit(10).unwrap();

        // look between the best battery and back
        for i in (second_battery_index..batteries.len()).rev() {
            let battery = batteries[i];
            let battery_joltage: u32 = battery.to_digit(10).unwrap();

            if battery_joltage > second_battery_joltage {
                second_battery_index = i;
                second_battery_joltage = battery_joltage;
            }
        }

        // let mut pretty_bank = String::from(bank);
        // let pretty_first = format!("[{}]", first_battery_joltage);
        // let pretty_second = format!("[{}]", second_battery_joltage);
        // pretty_bank.replace_range(
        //     first_battery_index..first_battery_index,
        //     pretty_first.as_str(),
        // );
        // pretty_bank.replace_range(
        //     second_battery_index..second_battery_index,
        //     &pretty_second.as_str(),
        // );
        // println!("bank: {}", bank);
        // println!("bank: {}", pretty_bank);
        // println!(
        //     "battery joltage: {}{} @ {},{}\n",
        //     first_battery_joltage,
        //     second_battery_joltage,
        //     first_battery_index,
        //     second_battery_index
        // );

        let joltage: usize = format!("{}{}", first_battery_joltage, second_battery_joltage)
            .parse()
            .unwrap();

        part1_total_joltage += joltage;
    });
    println!("part1 joltage: {}", part1_total_joltage);

    // part 2 is 12 batteries
    // i think we can look from left to right, selecting the biggest value possible
    // as long as this digit is leaves enough digits to the right of it to select the other batteries
    // for example if a 9 is the second to last digit it doesn't matter because there's still 11 batteries to get
    // so actually we can take the length of the batteries and only look at the first length-12 batteries joltage
    // if the biggest digit is the last one in this possible range we can just take the remaining batteries
    // if not we need to repeat this process, decreasing the length we check each time as we find the best joltage battery
    // e.g. length-11, length-10 etc until we either hit the last battery in this range or we get to the end of the batteries array
    let mut part2_total_joltage: usize = 0;

    input_string.split("\n").for_each(|bank| {
        const BATTERY_COUNT: usize = 12;

        let batteries: Vec<char> = bank.chars().collect();

        // (battery_index, battery_joltage)
        let mut highest_batteries: [(usize, u32); BATTERY_COUNT] = [
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ];

        highest_batteries[0].1 = batteries[0].to_digit(10).unwrap();

        //println!("bank: {}", bank);

        let mut previous_start_index = 0;

        for i in 0..BATTERY_COUNT {
            //println!("battery: {}", i);
            // if we're not the first index, start looking at batteries
            // that are after the previous one found
            if i > 0 {
                previous_start_index = highest_batteries[i - 1].0 + 1;
            }
            // println!(
            //     "looking from index {} to {}",
            //     previous_start_index,
            //     batteries.len() - 11 + i 
            // );
            for j in previous_start_index..batteries.len() - 11 + i {
                let battery_joltage: u32 = batteries[j].to_digit(10).unwrap();
                if battery_joltage > highest_batteries[i].1 {
                    //println!("found bigga battery @ {} with joltage: {}", j, battery_joltage);
                    highest_batteries[i].0 = j;
                    highest_batteries[i].1 = battery_joltage;
                }
            }
        }

        let joltage: usize = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            highest_batteries[0].1,
            highest_batteries[1].1,
            highest_batteries[2].1,
            highest_batteries[3].1,
            highest_batteries[4].1,
            highest_batteries[5].1,
            highest_batteries[6].1,
            highest_batteries[7].1,
            highest_batteries[8].1,
            highest_batteries[9].1,
            highest_batteries[10].1,
            highest_batteries[11].1
        )
        .parse()
        .unwrap();

        //println!("joltage: {}\n\n", joltage);

        part2_total_joltage += joltage;
    });

    // 170108965159310 too low
    println!("part2 joltage: {}", part2_total_joltage);
}
