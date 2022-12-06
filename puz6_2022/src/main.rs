use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    //Check if file exists
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(s) = line {
              let charlist = s.chars();
              //let mut tmp_charlist = String::from("____");  // <---- for first assignment
              let mut tmp_charlist = String::from("______________"); // <------ for second assignment
              let mut index: i32 = 0;

              //move through the characters in this list
              for character in charlist{
                  tmp_charlist.insert(0,character);
                  tmp_charlist.pop();

                  //check if all different
                  let unique:bool = check_unique(&tmp_charlist);            
                  if (unique == true){
                    println!("The list of characters: {:?}",tmp_charlist);
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

fn check_unique(tmp_charlist: &String) -> bool{
    //return true if all unique
    let mut unique = true;
    let charlist = tmp_charlist.chars();
    for character in charlist{
        let nmatch = tmp_charlist.matches(character).count();
        if (nmatch > 1 || tmp_charlist.contains('_')){
            unique = false;
        }
    }
    return unique;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}