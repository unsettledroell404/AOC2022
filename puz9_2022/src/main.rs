use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    //Check if file exists
    if let Ok(lines) = read_lines("./input.txt") {

        //create head location map and add the initial point
        let mut tail_locations: HashSet<(i32,i32)> = HashSet::new();
        let rope_length  = 10;//2 for assignment 1, 10 for assignment 2
        let mut current_knot_locations: Vec<(i32,i32)> = vec![(0,0);rope_length];
        tail_locations.insert(current_knot_locations[rope_length-1]);

        let mut head_locations: HashSet<(i32,i32)> = HashSet::new();
        head_locations.insert(current_knot_locations[0]);

        //hash map mapping the direction key to +-xy
        let direction_map: HashMap<char,(i32,i32)> = HashMap::from([
            ('U',(0,1)),
            ('D',(0,-1)),
            ('L',(-1,0)),
            ('R',(1,0)),
        ]);

        for line in lines {
            if let Ok(s) = line{
                let vec: Vec<&str> = s.split(" ").collect();
                let nsteps: i32 = vec[1].parse().unwrap();
                let direction = vec[0].chars().nth(0).unwrap();

                //loop nsteps times
                for n in 1..nsteps+1{
                    //store the current location of the head
                    let last_knot_locations = current_knot_locations.clone();
                    //get the new head position
                    current_knot_locations[0] = (current_knot_locations[0].0 + direction_map.get(&direction).unwrap().0 , current_knot_locations[0].1 + direction_map.get(&direction).unwrap().1);
                    head_locations.insert(current_knot_locations[0]);
                    //iterate over the knots
                    for i in 1..rope_length{
                        //work out if and where the tail shoud be moving. If it is not adjacent, it should move to the previous locaiton of the head
                        // sqrt((xa-xb)^2 + (ya-yb)^2)>sqrt(2)+e
                        if (((current_knot_locations[i-1].0-current_knot_locations[i].0)).pow(2) + ((current_knot_locations[i-1].1-current_knot_locations[i].1)).pow(2)) > 2{
                            current_knot_locations[i] = last_knot_locations[i-1];
                            //if it is the tail that just moved, do this
                            if i==rope_length-1{
                                tail_locations.insert(current_knot_locations[i]);
                            }
                        }
                    }
                }
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
