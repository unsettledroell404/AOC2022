use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    //Check if file exists
    if let Ok(lines) = read_lines("./input.txt") {

        //create head location map and add the initial point
        let mut head_locations: HashSet<(i32,i32)> = HashSet::new();
        let mut current_head_location: (i32,i32) = (0,0);
        head_locations.insert(current_head_location);

        let mut tail_locations: HashSet<(i32,i32)> = HashSet::new();
        let mut current_tail_location: (i32,i32) = (0,0);
        tail_locations.insert(current_tail_location);

        //hash map mapping the direction key to +-xy
        let direction_map: HashMap<char,(i32,i32)> = HashMap::from([
            ('U',(0,1)),
            ('D',(0,-1)),
            ('L',(-1,0)),
            ('R',(1,0)),
        ]);

        println!("{:?}",current_tail_location);

        for line in lines {
            if let Ok(s) = line{
                let nsteps = s.chars().nth(2).unwrap().to_digit(10).unwrap();
                let direction = s.chars().nth(0).unwrap();

                println!("to walk {:} to {:}",nsteps,direction);

                //loop nsteps times
                for n in 1..nsteps+1{
                    //store the current location of the head
                    let last_head_location = current_head_location;
                    //get the new head position
                    current_head_location.0 = current_head_location.0 + direction_map.get(&direction).unwrap().0;
                    current_head_location.1 = current_head_location.1 + direction_map.get(&direction).unwrap().1;
                    //add the visited head location
                    head_locations.insert(current_head_location);
                    //work out where the tail shoud be moving. If it is not adjacent, it should move to the previous locaiton of the head
                    // sqrt((xa-xb)^2 + (ya-yb)^2)>sqrt(2)+e
                    if (((current_head_location.0-current_tail_location.0) as f64).powf(2.0) + ((current_head_location.1-current_tail_location.1) as f64).powf(2.0)).powf(0.5)  > 1.42{
                        current_tail_location = last_head_location;
                        tail_locations.insert(current_tail_location);
                    }
                    println!("{:?}",current_tail_location);
                }
                //println!("{:}",nstepsmap.get
            }
        }
        println!("Number of spots the head visited is {:?}",head_locations.len());
        println!("Number of spots the tail visited is {:?}",tail_locations.len());
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
