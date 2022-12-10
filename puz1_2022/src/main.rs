use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BinaryHeap;

fn main() {
    //Check if file exists
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut heap = BinaryHeap::<i32>::new();
        let mut tmp_sum: i32 = 0;
        for line in lines {
            if let Ok(s) = line {
                if(s==""){
                    heap.push(tmp_sum);
                    tmp_sum=0;
                    continue
                }
                tmp_sum += s.parse::<i32>().unwrap();
            }
        }
        //top 1 elf carrying:
        let elf1 = heap.pop().unwrap();
        println!("Top elf is carrying {:} cookies",elf1);
        // top 3 elves are carrying:
        let elf2 = heap.pop().unwrap();
        let elf3 = heap.pop().unwrap();
        println!("Top 3 elves are carrying {:} cookies",elf1+elf2+elf3);
    }
    else{
        println!("Error reading file");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}