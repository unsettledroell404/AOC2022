use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //Check if file exists
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut total_score:i32 = 0;
        let mut secret_score:i32 = 0;
        for line in lines {
            if let Ok(read_string) = line {
                //println!("{}", &read_string);

                //give the string to the CheckOutcome function to get the score 
                let outcome_score:i32 = checkOutcome(&read_string);
                let shape_score:i32 = checkShapeScore(&read_string);
                total_score = total_score + outcome_score + shape_score; 
                
                //now we try the super secret strategy ...
                let scoreWithSecretOutcome:i32 = findCorrectMove(&read_string);
                secret_score = secret_score +  scoreWithSecretOutcome;
            }
        }
        println!("{}", total_score);
        println!("{}", secret_score);
    }
    else{
        println!("Error reading file");
    }
}

fn checkOutcome(read_string:&String) -> i32{
    // compare the entire strings, only 9 options anyway
    //draw:
    if(read_string == "A X" || read_string == "B Y" || read_string == "C Z"){
        return 3;
    //win:
    }
    else if(read_string == "A Y" || read_string == "B Z" || read_string == "C X"){
        return 6;
    //otherwise we lose
    }else{
        return 0;
    }
}

fn checkShapeScore(read_string:&String) -> i32{
    if(read_string.contains("X")){
        return 1;
    }
    else if(read_string.contains("Y")){
        return 2;
    }else{
        return 3;
    }
}

fn findCorrectMove(read_string:&String) -> i32{
    //find correct move according to super secret strategy
    if(read_string.contains("X")){
        if(read_string.contains("A")){
            //lose + scissors
            return 3;
        }
        if(read_string.contains("B")){
            //lose + rock
            return 1;
        }
        if(read_string.contains("C")){
            //lose + paper
            return 2;
        }
    }
    if(read_string.contains("Y")){
        if(read_string.contains("A")){
            //draw + rock
            return 4;
        }
        if(read_string.contains("B")){
            //draw + paper
            return 5;
        }
        if(read_string.contains("C")){
            //draw + scissors
            return 6;
        }
    }
    if(read_string.contains("Z")){
        if(read_string.contains("A")){
            //win + scissors
            return 8;
        }
        if(read_string.contains("B")){
            //win + rock
            return 9;
        }
        if(read_string.contains("C")){
            //win + paper
            return 7;
        }
    }
    return 0




}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}