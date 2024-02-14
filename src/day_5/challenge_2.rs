use std::fs;
use std::cmp::max;
use std::cmp::min;
#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
struct SeedRange
{
    start: i64,
    end: i64
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
struct ConversionMap
{
    destination: i64,
    origin: i64,
    range: i64
}

pub fn run()
{   
    
    // let file = fs::read_to_string("./src/day_5/input.example.txt").expect("Read File");
    let file = fs::read_to_string("./src/day_5/input.txt").expect("Read File");

    let rows: Vec<&str> = file.split("\n\n").collect();

    let seed_intervals: Vec<i64> = get_seed_intervals(&rows);

    let mut seed_ranges: Vec<SeedRange> = get_seed_ranges(seed_intervals);

    let conversion_maps: Vec<Vec<ConversionMap>> = get_conversion_maps(rows);

    for map in conversion_maps
    {
        let mut new_ranges: Vec<SeedRange> = Vec::new();
        
        for m in map
        {
            let mut no_overlaping_ranges: Vec<Vec<SeedRange>> = Vec::new();

            for seed in &seed_ranges
            { 
                let ( converted, remaining ) = convertion(m.clone(), seed.clone());   

                if(converted).is_some()
                {
                    let new_range: SeedRange = converted.unwrap();

                    new_ranges.push(new_range);
                }

                no_overlaping_ranges.push(remaining);
            }

            seed_ranges = no_overlaping_ranges.concat();
        }

        new_ranges.extend(seed_ranges);

        seed_ranges = new_ranges;
    }
    
    seed_ranges.sort_by(|a, b| a.start.cmp(&b.start));

    println!("{}", min(seed_ranges[0].start, seed_ranges[0].end));
}

fn get_seed_intervals(arr: &Vec<&str>) -> Vec<i64>
{
    arr[0][7..].split_whitespace().filter_map(|i| i.parse::<i64>().ok()).collect()
}

fn get_seed_ranges(seed_intervals: Vec<i64>) -> Vec<SeedRange>
{
    let mut seed_ranges: Vec<SeedRange> = Vec::new();

    for i in 0..seed_intervals.len() / 2
    {
        let start = seed_intervals[i * 2];
        
        let length = seed_intervals[i * 2 + 1];

        let end = start + length;
        
        seed_ranges.push(SeedRange {
            start,
            end
        })
    }

    seed_ranges
}

fn get_conversion_maps(map: Vec<&str>) -> Vec<Vec<ConversionMap>>
{
    let mut conversion_maps: Vec<Vec<ConversionMap>> = Vec::new();

    map.iter()
        .skip(1)
        .flat_map(|m| m.split(":\n").nth(1))
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|fm|
            ({
                let mut conversion_map: Vec<ConversionMap> = Vec::new();

                fm.split("\n")
                    .for_each(|fmi|({
                        let mut arr: Vec<i64> = Vec::new();
    
                        fmi.split_whitespace()
                            .into_iter()
                            .for_each(|fmip| arr.push(fmip.parse::<i64>().unwrap()));
    
                        conversion_map.push(ConversionMap {
                            destination: arr[0],
                            origin: arr[1],
                            range: arr[2]
                        })
                    }));

                    conversion_maps.push(conversion_map)
            })
        );

        conversion_maps
}

fn convertion(ConversionMap { destination, origin, range }: ConversionMap, SeedRange { start, end }: SeedRange) -> (Option<SeedRange>, Vec<SeedRange>)
{
    let map_end = origin + range - 1;

    let overlap_start = max(start, origin);
    
    let overlap_end = min(end, map_end);

    if overlap_start > overlap_end
    {
        return (None, vec![SeedRange { start, end }])
    }

    let converted_start = overlap_start + destination - origin;

    let converted_end = overlap_end + destination - origin;

    let mut seed_less_than_map: Vec<SeedRange> = Vec::new();

    let mut seed_greater_than_map: Vec<SeedRange> = Vec::new();

    if overlap_start > start
    {
        seed_less_than_map.push(SeedRange { start, end: overlap_start - 1 });
    }

    if overlap_end < end
    {
        seed_greater_than_map.push(SeedRange { start: overlap_end + 1, end });
    }

    let over_values: Vec<SeedRange> = [&seed_less_than_map[..], &seed_greater_than_map[..]].concat();

    (Some(SeedRange { start: converted_start, end: converted_end }), over_values)

}

