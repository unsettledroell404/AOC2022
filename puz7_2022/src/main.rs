use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    //Check if file exists
    if let Ok(lines) = read_lines("./input.txt") {

        //to contain the current path
        let mut path:Vec<String> = Vec::new();//it does not work if it is &str, something with lifetimes idk
        //to contain the nested directory
        let mut dirsizes: HashMap<Vec<String>,i32> = HashMap::new();

        for line in lines {

            if let Ok(s) = line {
                 let v: Vec<&str> = s.split(|c| c == ' ').collect();

                 //take car of operations with $
                 if v[0]=="$" && v[1]=="cd" && !(v[2] == ".."){
                     path.push(v[2].to_string());
                     // set hashmap size to 0
                     dirsizes.insert(path.clone(),0);
                     continue
                 }
                 if v[0]=="$" && v[1]=="cd" && v[2] == ".."{
                     path.pop();
                     continue
                 }
                 if v[0]=="$" && v[1]=="ls" {
                     //do nothing
                     continue
                 }

                 assert!(!(v[0]=="$"));

                 //now we can enter sizes
                 //if it is dir, we do nothing
                 //if it is size, add the size to all folders
                 if(v[0]=="dir"){
                     //do nothing
                     continue
                 }
                 //only a number can exist now.
                 //below adds folder size to all parents.
                 let mut tmpworkdir = path.clone();
                 while(&tmpworkdir.len() > &0){
                     dirsizes.entry(tmpworkdir.clone()).and_modify(|size| *size+=v[0].parse::<i32>().unwrap());
                     tmpworkdir.pop();
                 }
            }
        }


        //part 1 find als dirs with <10k size
        let mut sumsizes = 0;
        for dir in dirsizes.iter(){
            if *dir.1<100000 && *dir.1!=0{
                sumsizes+=dir.1;
            }  
        }
        println!("sum sizes: {:?}",sumsizes);

        //part 2 
        //get the total used space:
        let homedir = vec![String::from("/")];
        let num = dirsizes.get(&homedir).unwrap();
        let unused_space = 70000000-num;
        let space_to_free = 30000000-unused_space;
        println!("space to free: {:?}",space_to_free);
        
        //find directory with the size larger than and closest to space_to_free
        let mut best_excess: i32 = 700000000;
        let mut best_size: i32 = 0;
        for dir in dirsizes.iter(){
            let excess = *dir.1 - space_to_free;
            if excess>=0 && excess<best_excess{
                best_excess = excess;
                best_size = *dir.1;
            }  
        }
        println!("best to delete size: {:?}",best_size);
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
