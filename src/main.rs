use std::{collections::HashMap, fs};

#[ derive(Debug)]
struct Range{
    lower:u64,
    upper:u64,
}

fn read_input(filename: String) -> (Vec<Range>, Vec<u64>){
    let contents = fs::read_to_string(filename).expect("File exists");

    let mut ranges : Vec<Range> = Vec::new();
    let mut ids : Vec<u64> = Vec::new();

    for line in contents.split("\n"){
        if line.trim().len() == 0{
            continue
        }
        if line.contains("-"){
            println!("line = |{}|", line);
            let parts = line.split("-").map(
                |s|  u64::from_str_radix(s, 10).expect("parsable int")
            );
            ranges.push(
                Range { 
                    lower: parts.clone().nth(0).expect("pre -"), 
                    upper: parts.clone().nth(1).expect("post -"),
                }
            );
        }
        else {
            ids.push(
                u64::from_str_radix(line, 10).expect("parasbale")
            )
        }
    };

    return (ranges, ids);

}

fn test_ranges(n: u64, ranges:&Vec<Range>) -> bool {
    for range in ranges.iter(){
        if range.lower <= n && n <= range.upper{
            return true;
        }
    }
    return false;
}

fn _part_1(){
    // let filename = "example.txt".to_string();
    let filename = "input.txt".to_string();

    let (ranges, ids) = read_input(filename);

    let mut fresh_count = 0;

    for id in ids.iter(){
        if test_ranges(*id, &ranges){
            fresh_count += 1;
        }
    }
    println!("Fresh: {}", fresh_count);
}

fn merge_ranges(ranges: &Vec<Range>) -> Vec<Range>{
    // Merge ranges
    // Sort lower bounds and upper bounds in ascending order
    // From smallest to largest iterate through, track nesting level
    
    let mut bounds_map : Vec<(u64, i8)> = Vec::new();
    
    for range in ranges.iter(){
        bounds_map.push((range.lower, 1));
        bounds_map.push((range.upper+1, -1));
    }
    
    bounds_map.sort_by(|a, b| a.0.cmp(&b.0));
    
    let mut new_ranges: Vec<Range> = Vec::new();
    let mut current_lower= 0;
    let mut nesting_level = 0;

    for (bound, delta_nest) in bounds_map.iter(){
        if nesting_level == 0 {
            current_lower = *bound;
        }
        nesting_level += delta_nest;
        if nesting_level == 0 {
            new_ranges.push({
                Range { lower: current_lower, upper: *bound }
            })
        }
    }

    return new_ranges;
}

fn measure_ranges(ranges: Vec<Range>) -> u64{
    return ranges.iter().map(|r| r.upper - r.lower).sum()
}

fn main() {
    // let filename = "example.txt".to_string();
    let filename = "input.txt".to_string();

    let (ranges, _ids) = read_input(filename);

    let merged_ranges = merge_ranges(&ranges);
    
    println!("Merged {:?}", merged_ranges);

    let fresh_count = measure_ranges(merged_ranges);

    println!("Fresh: {}", fresh_count);
}
