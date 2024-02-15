use std::fs;

use regex::Regex;


pub fn run()
{
    // let input = fs::read_to_string("./src/day_6/input.example.txt").expect("Attempt to read file");

    let input = fs::read_to_string("./src/day_6/input.txt").expect("Attempt to read file");

    let treated_input: Vec<&str> = input.split("\n").collect();

    let reg = Regex::new(r"\s+").expect("Attempt to compile an Regex");

    let travels_time: Vec<i32> = reg.replace_all(treated_input[0].split(":")
        .collect::<Vec<&str>>()[1]
        .trim(), ",")
        .split(",")
        .map(|t| t.parse::<i32>().unwrap())
        .collect();

    let travels_distance: Vec<i32> = reg.replace_all(treated_input[1].split(":")
        .collect::<Vec<&str>>()[1]
        .trim(), ",")
        .split(",")
        .map(|t| t.parse::<i32>().unwrap())
        .collect();

    let mut many_ways_to_win: Vec<i32> = Vec::new();

    for (i, time) in travels_time.iter().enumerate()
    {   
        let mut ways_to_win_qty: i32 = 0;

        let mut first_match = false;

        for y in 1..*time + 1
        {
            let seconds_button_press = y;


            let time_remaining = time - seconds_button_press;

            let total_travel_distance = seconds_button_press * time_remaining;

            if total_travel_distance < travels_distance[i] && first_match
            {
                break
            }

            if total_travel_distance > travels_distance[i]
            {
                ways_to_win_qty += 1;

                first_match = true;
            }

            // println!(
            //     "\
            //         Segundos pressionado {}\n\
            //         Tempo total {}\n\
            //         Tempo Restante {}
            //     "
            // , seconds_button_press, time, time_remaining);
        }

        many_ways_to_win.push(ways_to_win_qty);   
    }

    println!("{:?}", many_ways_to_win.iter().fold(1, |acc, n| acc * n));
}