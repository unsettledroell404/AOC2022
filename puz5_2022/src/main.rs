use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    //make a vector containing the stacks
    let mut vec= Vec::new();

    //read the crates
    //Check if file exists
    for i in 0..9{
        if let Ok(lines) = read_lines("./input_crates.txt") {
            let mut s2:String = String::from("");
            for line in lines {
                if let Ok(s) = line {
                let c = s.chars().nth(1+4*i).unwrap();
                if (c!=' '){
                    s2.insert(0,c);
                }
            }
        }
        //println!("{}",&s2);
        let char_vec: Vec<char> = s2.chars().collect();
        vec.push(char_vec.clone());
        }
    else{
        println!("Error reading file");
        }
    }
    

    //read the moves
    //Check if file exists
    let mut vec_multiple = vec.clone();

    if let Ok(lines) = read_lines("./input_procedure.txt") {       
        for line in lines {
            if let Ok(s) = line {
                let v = parse_line(s);

                //1-at-a-time
                for i in 0..v[0]{
                    //add item to correct line
                    let character:char = vec[v[1]-1].last().unwrap().clone();
                    vec[v[2]-1].push(character);
                    vec[v[1]-1].pop();
                }

                //multiple-at-a-time
                let length = vec_multiple[v[1]-1].len();
                for i in 0..v[0]{
                    //add item to correct line
                    let character:char = vec_multiple[v[1]-1][length-v[0]+i].clone();
                    vec_multiple[v[2]-1].push(character);
                }
                vec_multiple[v[1]-1].truncate(length-v[0]);
            }
        }
        //what is on top of each stack?
        for i in 0..9{
        println!("Top of all stacks part 1: {:?}",vec[i].last().unwrap());
        }
        for i in 0..9{
        println!("Top of all stacks part 2: {:?}",vec_multiple[i].last().unwrap());
        }
    }
    else{
        println!("Error reading file");
    }
}


fn parse_line(s: String) -> Vec<usize>{
    let s2 = s.replace("move ", "");
    let s3 = s2.replace(" from ", " ");
    let s4 = s3.replace(" to ", " ");
    let v: Vec<&str> = s4.split(|c| c == ' ').collect();
    let V: Vec<usize> = v.iter().map(|c| c.parse().unwrap()).collect();
    return V
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}