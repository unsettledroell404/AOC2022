use std::collections::BinaryHeap;
//use eval::{eval,to_value};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate evalexpr;
use evalexpr::*;

//declare monke struct
struct Monke {
    items:  Vec::<i64>,
    operation: char,
    operation_value: i64,
    divisor: i64,
    true_monke: usize, 
    false_monke: usize,
    inspections: i64,
}

//monke has an implementation
impl Monke {
    //constructor
    fn new_monke(items: Vec::<i64>,operation: char, operation_value: i64, divisor: i64,true_monke: usize,false_monke: usize) -> Self{
        Self{
            items: items,
            operation: operation,
            operation_value: operation_value,
            divisor: divisor,
            true_monke: true_monke, 
            false_monke: false_monke,
            inspections: 0,
        }
    }

    //do a monke, return the monkey nummer and item value 
    fn process(&mut self) -> (usize,i64) {
        let mut tmp_item = self.items[0];
        //the item's worry level is changed
        if self.operation == '+'{
            if self.operation_value == 0{
                tmp_item = tmp_item + tmp_item;
            }else{
                 tmp_item = tmp_item + self.operation_value;
            }
        }
        else if self.operation == '*'{
            if self.operation_value == 0{
                tmp_item = tmp_item * tmp_item;
            }else{
                tmp_item = tmp_item * self.operation_value;
            }
        }else{
            panic!("bad operation")
        }
        //monke bored, divide by common divisor
        //tmp_item = ( tmp_item/3 ); //<---for part 1

        //test to which monkey we should throw
        //becomes funny wihth 64 .. test if it is very close to whole number
        self.items.remove(0);
        self.inspections +=1;
        if tmp_item % self.divisor == 0{
            //truemonke
            //monke ditched the item! pop first
            return(self.true_monke, tmp_item)
        }
        else{
            //falsemonke
            //monke ditched the item! pop first
            return(self.false_monke, tmp_item)
        }
    }
}

fn main() {

    let mut monkevec: Vec<Monke> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        //placeholders to make monke
        let mut items: Vec::<i64> = Vec::new();
        let mut operation: char = 'x';
        let mut operation_value: i64 = 0;
        let mut divisor: i64 = 1;
        let mut true_monke: usize = 0;
        let mut false_monke: usize = 0;

        for line in lines {
            if let Ok(s) = line{
                let v: Vec<&str> = s.split(": ").collect();
                    
                    if v[0]=="Monkey"{
                        //new monke
                        //nothing
                    }
                    else if v[0] == "  Starting items" 
                    {
                        items = v[1].split(", ").map(|v| v.parse::<i64>().unwrap()).collect();
                    }
                    else if v[0] == "  Operation" 
                    {
                        let tmp: Vec<&str> = v[1].split(" ").collect();
                        println!("{:?}",tmp);
                        operation = tmp[3].chars().nth(0).unwrap();
                        println!("{:?}",operation);
                        if tmp[4] == "old"{
                            operation_value = 0;
                        }else{
                            operation_value = tmp[4].parse::<i64>().unwrap();
                        }
                    }
                    else if v[0] == "  Test" 
                    {
                        let tmp: Vec<&str> = v[1].split(" by ").collect();
                        divisor = tmp[1].parse::<i64>().unwrap();
                    }
                    else if v[0] == "    If true" 
                    {
                        let tmp: Vec<&str> = v[1].split(" monkey ").collect();
                        true_monke = tmp[1].parse::<usize>().unwrap();
                    }
                    else if v[0] == "    If false" 
                    {
                        let tmp: Vec<&str> = v[1].split(" monkey ").collect();
                        false_monke = tmp[1].parse::<usize>().unwrap();

                        //done building monke: add to the list
                        let monke = Monke::new_monke(items.clone(),operation,operation_value,divisor,true_monke,false_monke);
                        monkevec.push(monke);
                    }
            }
        }
    }

    println!("lenght= {:?}",monkevec.len());

    //let prod = 1; //<---part1
    let prod: i64 = monkevec.iter().map(|m| m.divisor).product();
    println!("Common divisor is {:}",prod);

    let iterations: i32 = 10000;//<--20 for part 1
    for _i in 0..iterations{
        for monke_now in 0..monkevec.len(){
            //monke throws away all items each time
            while monkevec[monke_now].items.len() > 0{            
                let (monke_to, monke_item) = monkevec[monke_now].process();
                monkevec[monke_to].items.push(monke_item%prod);//<---here is the magic part to stop it from overflowing
            }
        }
    }

   //get the highest two monke scores to get the monke business
   let mut inspectionsheap = BinaryHeap::<i64>::new();
    for monke_now in 0..monkevec.len(){
        inspectionsheap.push(monkevec[monke_now].inspections)
    }
   println!("monke business: {:?}",inspectionsheap.pop().unwrap()*inspectionsheap.pop().unwrap());
   // 17673687232
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
