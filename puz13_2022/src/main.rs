//use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashSet;

struct Packet{
    list: String,
}

impl Packet{
    fn isList(&self) -> bool{
        self.list.chars().nth(0)==Some('[')
    }
    fn isInt(&self) -> bool{
        self.list.chars().nth(0)!=Some('[')
    }
}


fn main() {
    //let mut grid :Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./input_test.txt") {
        let mut counter = 0;
        let mut pair_counter: i32 = 1;
        let mut line2 = String::new();
        let mut line1 = String::new();
        let mut true_indexes: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(s) = line{
                if counter==2{
                    //do the do
                    println!("{:}",&line1);
                    println!("{:}",&line2);
                    println!("");

                    //put in packets
                    let mut packet1 = Packet{list: line1.clone()};
                    let mut packet2 = Packet{list: line2.clone()};

                    if compare(packet1,packet2) == true{
                        true_indexes.push(pair_counter);
                    }
                    pair_counter+=1;
                    counter=0;
                }
                else if counter==1{
                    line2 = s;
                     counter+=1;
                }
                else{
                    line1 = s;
                    counter+=1;
                }

            }
        }
        println!("true indexes: {:?}",true_indexes)
    }
}

fn compare(mut packet1: Packet, mut packet2: Packet,) -> bool{
    //decrease depth 1 level, are we dealing with numbers now?
    println!("to compare {:} and {:}", packet1.list, packet2.list);

    let rightorder: bool;
    let mut indexes = get_part_indexes(&packet1.list);

    let mut toCompare_list1: String = String::new();
    let mut idxnow: usize = indexes.1;
    for i in indexes.0..indexes.1{
        toCompare_list1.insert(0,packet1.list.remove(idxnow));
        idxnow-=1;
    }
    toCompare_list1.insert(0,packet1.list.remove(indexes.0));//hack

    indexes = get_part_indexes(&packet2.list);
    let mut toCompare_list2: String = String::new();
    idxnow = indexes.1;
    for i in indexes.0..indexes.1{
        toCompare_list2.insert(0,packet2.list.remove(idxnow));
        idxnow-=1;
    }
    toCompare_list2.insert(0,packet2.list.remove(indexes.0));//hack
    println!("split part {:}{:}", toCompare_list1,toCompare_list2); 

        let positionopt = toCompare_list1.chars().zip(toCompare_list2.chars()).position(|(p1, p2)| (p1 as u8) != (p2 as u8));
        
        if let Some(position) = positionopt{//the first list was shorter OR < was found
            rightorder = toCompare_list1.chars().nth(position).unwrap()<toCompare_list2.chars().nth(position).unwrap();
            println!("rightorder: {:}", rightorder);
        }else{//no uneq found or second list longer -> is false
            rightorder = false;
            println!("rightorder: {:}", rightorder);
        }

    //make them into lists if the aren't
    if packet1.isList() && packet2.isInt(){
        packet2.list.insert(0,'[');
        packet2.list.push(']');
    }
    if packet2.isList() && packet1.isInt(){
        packet1.list.insert(0,'[');
        packet1.list.push(']');
    }

    // we done?
    if packet1.list.len()>0 || packet2.list.len()>0{
         // recursive time
          compare(packet1,packet2);
    }
    rightorder
}

fn get_part_indexes(list: &String) -> (usize,usize) {
    let pos1 = list.chars().position(|c| c==']').unwrap();
    let mut pos2 = pos1;
    let mut t = pos1;
    for elem in list.chars().rev(){
        if list.chars().nth(t).unwrap() == '['{
            pos2 = t;
            break;
        }
        t-=1;
    }
    (pos2,pos1)
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
