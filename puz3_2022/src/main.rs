use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //Check if file exists
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut prioSum:i32 = 0;
        let mut prioSum_pergroup:i32 = 0;
        //things for part 2:
        let mut line_number = 0;
        let mut str1:String = String::from("Hello");
        let mut str2:String = String::from("Hello");
        let mut str3:String = String::from("Hello");

        
        for line in lines {

            line_number+=1;

            if let Ok(s) = line {
                //println!("{}", &read_string);

                //find the characters that are the same
                let length:usize = s.chars().count();
                if length%2 != 0{
                    panic!();//length should always be factor of 2
                }
                // use slices, they are cool
                let sl1 = &s[..length/2];
                let sl2 = &s[length/2..];
                //compare the slices to find the same character
                //using an iterator over c that returns the index of matching element if any element satisfies (sl1 contains sl2(c))
                let result = sl1.chars().position(|c| sl2.contains(c)).unwrap();
                //println!("{:?}", result);
                //the character is
                let character = sl1.chars().nth(result).unwrap();
                let prio = calcPrio(character);

                prioSum = prioSum + prio;



                //part 2: fill string 1-3 while iterating
                println!("line number {}",line_number);
                if line_number%3==1 //line 1
                {
                    str1 = s.clone();
                }
                if line_number%3==2 //line 2
                {
                    str2 = s.clone();
                }
                if line_number%3==0 //line 3
                {
                    str3 = s.clone();
                    //do the comparisons: what item appears in all 3 rucksacks?
                   let index =  str1.chars().position(|c| str2.contains(c)&&str3.contains(c)).unwrap();
                   let groupcharacter = str1.chars().nth(index).unwrap();
                   //println!("letter is {}",groupcharacter);
                   prioSum_pergroup = prioSum_pergroup + calcPrio(groupcharacter);
                }


            }
        }
        println!("Priority sum: {:?}", prioSum);
        println!("Group character sum: {:?}", prioSum_pergroup);
    }
    else{
        println!("Error reading file");
    }
}

fn calcPrio(character: char) -> i32{
    //convert to a priority. lower characters:
    let mut prio:i32 = (character as i32)-64;
    //upper case A has prio 1 but should have prio 27
    prio = prio + 26;
    //the lower cases are too high prioritized now, so fix this
    if (prio>52)//it is lower case
    {
        prio = prio-58;
    }
    return prio
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}