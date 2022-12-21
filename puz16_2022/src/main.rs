//use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs;

use std::collections::HashMap;



fn main() {
    //Check if file exists
    if let Ok(lines) = read_lines("./input_test.txt") {
        //list of valves
        let mut valves: Vec<String> = Vec::new(); 
        let mut neighbours: Vec<Vec<String>> = Vec::new(); 
        let mut rate: Vec<i32> = Vec::new(); 

        for line in lines {
            if let Ok(read_string) = line {
                let v: Vec<&str> = read_string.split(" has flow rate=").collect();
                let name = v[0].replace("Valve ","");
                let flow_rate: i32 = v[1].split(";").collect::<Vec<&str>>()[0].parse().unwrap();
                let leads_to = v[1].split("valve").collect::<Vec<&str>>()[1];
                let leads_to = leads_to.replace("s","").replace(" ","");
                let leads_to: Vec<&str> = leads_to.split(",").collect();
                let leads_to: Vec<String> = leads_to.iter().map(|v| v.to_string()).collect();

                //store in a list of sorts
                valves.push(name);
                neighbours.push(leads_to);
                rate.push(flow_rate);

            }
        }
        //println!("valve:{:?}",&valves);
        //println!("rate:{:?}",&rate);
        //println!("leads to:{:?}",&neighbours);

        let opened_list :Vec<String>= Vec::new();
        let time = 26;  //30 pt 1, 26 pt 2
        let elephant = true; //false pt 1, true pt 2
         
        let mut position = 0;
        //start is AA
        if let Some(idx) = valves.iter().position(|r| *r=="AA")
        {
            position = idx;
            println!("start position = {}", position);
        }else{
            panic!("this should never happen. Trying to find AA failed.");
        }

        //global cache
        let mut cache: HashMap<String, i32> = HashMap::with_capacity(7000000);
        println!("solved: max pressure release is: {:}" ,solve(position,time,elephant,&opened_list,&rate,&valves,&neighbours,&mut cache));
        println!("cache size is {}",cache.len());
    }
    else{
        println!("Error reading file");
    }

}

fn solve(position: usize, time: i32, elephant: bool, opened_list: &Vec<String>,rate: &Vec<i32>,valves: &Vec<String>,neighbours: &Vec<Vec<String>>,cache: &mut HashMap<String,i32>) -> i32{
    //has the function been called with the arguments: position, time, opened_list already? Then return the known answer
    //neighbours is never changed so it does not need to be checked.


    let mut argument_key: String = opened_list.join("-");
    argument_key.push_str("-");
    argument_key.push_str(&(position.to_string()));
    argument_key.push_str("-");//this is VERY important. Leaving it out will mess with the keys because it is unknown if the number belongs to position or time.
    argument_key.push_str(&(time.to_string()));
    argument_key.push_str("-");//adding this fixes part 2 for the test, but it fills the memory completely for the real input. Why?
    argument_key.push_str(&(elephant.to_string()));

    //if this is in the cache, return that.
    if let Some(value) = cache.get(&argument_key){
        return *value;
    }

    if time==0{
        if elephant == true && opened_list.len()>0 { //I assume at least 1 valve should be open at this point for any of this to make sense
            //elephant to start at AA
            let position_elephant;
            if let Some(idx) = valves.iter().position(|r| *r=="AA")
                {
                position_elephant = idx;
            }else{
                panic!("this should never happen. Trying to find AA failed.");
            }
            let ele_score = solve(position_elephant, 26, false, &opened_list,&rate,&valves,&neighbours,cache);
        }
        return 0;
    }
  
    let mut score = 0;
    
    //check score if we moved to another node
    for i in 0..neighbours[position].len(){
        let index_to_move;
        if let Some(idx) = valves.iter().position(|r| *r==neighbours[position][i])
        {
            index_to_move = idx;
        }else{
            index_to_move = 0;
            panic!("this should never happen. Trying to find {} failed.",neighbours[position][i]);
        }
        let tmp_score = solve(index_to_move, time-1, elephant, opened_list,&rate,&valves,&neighbours,cache);//get the right neighbours here
        if tmp_score>score
        {
            score = tmp_score;
        }
    } 

    //calculate the score of opening the current valve
    if rate[position] > 0 && !opened_list.contains(&valves[position]){//the flow is not 0 and we have not yet opened the valve
        let mut new_opened_list = opened_list.clone();
        new_opened_list.push(valves[position].clone());
        let tmp_score = (time-1) * rate[position] + solve(position, time-1, elephant, &new_opened_list,&rate,&valves,&neighbours,cache);
        let eq = (time-1) * rate[position];
        if tmp_score>score
        {
            score = tmp_score;
        }
    }
    //add to the cache and return
    cache.insert(argument_key, score);
    return score

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}