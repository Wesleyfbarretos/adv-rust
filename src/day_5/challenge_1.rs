// use crate::day_5::input::input;

// pub fn run()
// {   
    
//     let rows: Vec<&str> = input().split("\n\n").collect();

//     let seeds: Vec<i64> = get_seeds(&rows[0]);
    
//     let map: Vec<&str>  = get_map(rows);
    
//     let map_attributes: [&str;7] = [
//         "seed-to-soil",
//         "soil-to-fertilizer",
//         "fertilizer-to-water",
//         "water-to-light",
//         "light-to-temperature",
//         "temperature-to-humidity",
//         "humidity-to-location",
//     ];


//     let mut all_locations: Vec<i64> = Vec::new();

//     for seed in seeds {
// // 
//         all_locations.push(find_location(&map, seed, map_attributes));

//     }

//     let lower_location = all_locations.iter().min();

//     println!("Lower location: {:?}", lower_location.unwrap());

// }

// fn get_seeds(line: &str) -> Vec<i64>
// {
//     line[7..].split(' ')
//         .map(|c| c.parse::<i64>().unwrap())
//         .collect()
// }

// fn get_map(rows: Vec<&str>) -> Vec<&str> 
// {
//     rows[1..].iter()
//         .flat_map(|&c|{
//             c.split(":\n")
//             .skip(1)
//         })
//         .collect()
// }

// fn find_location(map: &Vec<&str>, seed: i64, map_attributes: [&str;7]) -> i64
// {

//     let mut location = seed;

//     for (index, _) in map_attributes.iter().enumerate()  {

//         let map_rows: Vec<&str> = map[index].split('\n').collect();
        
//         for c in map_rows.iter() {

//             let map_row: Vec<i64> = c.split(" ").flat_map(|c| c.parse::<i64>()).collect::<Vec<i64>>();

//             let destiny = map_row[0];

//             let origin = map_row[1];

//             let range = map_row[2];

//             if location >= origin && (origin + range - 1) >= location {

//                 location = (location - origin) + destiny;

//                 break
//             }

//         }

//     }

//     location
// }



