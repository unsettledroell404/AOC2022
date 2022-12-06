use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    //Check if file exists
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(s) = line {

              let charlist = s.chars();
              let mut unique_sequence = String::new();
              let mut index: i32 = 0;
              let sequence_length:usize = 14;//4 for part 1, 14 for part 2

              //move through the characters in this list. Build a unique sequence
              for character in charlist{
                  unique_sequence.insert(0,character);                                          //insert the new character
                  unique_sequence.truncate(sequence_length);                                    //the unique sequence can never be longer than sequence_length characters
                  let mut char_vec: Vec<char> = unique_sequence.chars().collect::<Vec<char>>(); //turn it into a vector of character which has the sort() and dedup() functions
                  char_vec.sort();                                                              //sort the characters: required for deduplication.
                  char_vec.dedup();                                                             //deduplicate the characters.
                  //check the length: if we reached sequence_length, we have succeeded.
                  if (char_vec.len()==sequence_length){
                    println!("The list of characters: {:?}",unique_sequence);
                    println!("The index: {:?}",index+1);
                    break;
                  }
                  index+=1;
              }
            }
        }
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