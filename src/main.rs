use std::fs;

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

fn main() {
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
