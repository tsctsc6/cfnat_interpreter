use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let mut hashmap: HashMap<String, i32> = HashMap::new();
    let input_file = File::open("input.txt").unwrap();
    let mut output_file = File::create("output.txt").unwrap();
    let buffered = BufReader::new(input_file);
    for line in buffered.lines() {
        let line = line.unwrap();
        let properties = line.split(" | ").collect::<Vec<&str>>();
        output_file.write(properties[0].as_bytes()).unwrap();
        output_file.write(":443#".as_bytes()).unwrap();
        output_file.write(properties[1].as_bytes()).unwrap();
        let mut count = -1;
        match hashmap.get_mut(properties[1])
        {
            Some(val) => {
                *val = *val + 1;
                count = *val;
            }
            None => {hashmap.insert(
                String::from(properties[1]), 0);
                count = 0;
            }
        }
        output_file.write(format!("-{}", count).as_bytes()).unwrap();
        output_file.write("\n".as_bytes()).unwrap();
    }
}
