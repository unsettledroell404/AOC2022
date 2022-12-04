use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //Check if file exists
    if let Ok(lines) = read_lines("./input.txt") {
        let mut num_fully_contained:i32 = 0;
        let mut num_overapping:i32 = 0;
        for line in lines {
            if let Ok(s) = line {
               let a = parse_line(&s);
               //println!("{:?}",a)
               if(is_fully_contained(&a)){
                   num_fully_contained+=1;
               }
               if(is_any_overlap(&a)){
                   num_overapping+=1;
               }
            }

        }
        println!("fully contained sets: {}",num_fully_contained);
        println!("overlapping sets: {}",num_overapping);
    }
    else{
        println!("Error reading file");
    }
}

fn parse_line(line: &String) -> Vec<i32>{
    //split into numbers
    let v: Vec<&str> = line.split(|c| c == ',' || c == '-').collect();
    //return as vector containing i32
    let V = v.iter().map(|c| c.parse().unwrap()).collect();
    return V;
}
fn is_fully_contained(v: &Vec<i32>) -> bool{
    let sections1 = &v[..2];
    let sections2 = &v[2..];
    //does section1 contain section2?
    let contained:bool = (sections1[0]>=sections2[0] && sections1[1]<=sections2[1] || sections2[0]>=sections1[0] && sections2[1]<=sections1[1]);
    return contained;
}
fn is_any_overlap(v: &Vec<i32>) -> bool{
    let sections1 = &v[..2];
    let sections2 = &v[2..];
    //is there any overlap? entire range1 below or entire range1 above?
    let overlap:bool = !(sections1[1]<sections2[0] || sections2[1]<sections1[0]);
    return overlap;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}