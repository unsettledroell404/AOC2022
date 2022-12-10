use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;
use std::io::Write;

fn main() {
    //Check if file exists
    if let Ok(lines) = read_lines("./input.txt") {

        //hash map mapping the ops to cycle times
        let cycle_map: HashMap<&str,i32> = HashMap::from([
            ("noop",1),
            ("addx",2),
        ]);
        //has set with the wanted locations to read the x value
        let wanted_cpu_positions: HashSet<i32> = HashSet::from([
            20,
            60,
            100,
            140,
            180,
            220,
        ]);
        //value of the register
        let mut x :i32 = 1;
        //the current iteration
        let mut cycle = 1;
        let mut totalscore=0;
        //plot the intial point
        print!("#");

        for line in lines {
            if let Ok(s) = line{
                let vec: Vec<&str> = s.split(" ").collect();
                let instruction = vec[0];

                //loop the number of times accoring to the map of the cycle times
                let required_cycles = *cycle_map.get(&instruction).unwrap();

                for n in 1..= required_cycles{
                    if wanted_cpu_positions.contains(&cycle){
                        totalscore = totalscore + x*cycle;
                    }
                    if instruction == "addx" && n == required_cycles{
                        x = x + vec[1].parse::<i32>().unwrap();
                    }

                    //move to the next row
                    if (cycle%40==0){
                        print!("\n");
                    }
                    //check if sprite overlaps with the cycle number and print result
                    if (cycle%40-x).abs()<=1{
                        print!("#");
                    }else{
                        print!(".");
                    }   
                    std::io::stdout().flush().unwrap();
                    
                    cycle+=1;

                }
            }

        }
        //print the result
        println!("Total score: {:}",totalscore);
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
