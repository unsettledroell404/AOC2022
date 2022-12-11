//declare monke struct
struct Monke {
    items:  Vec::<f64>,
    operation: char,
    operation_value: f64,
    divisor: f64,
    true_monke: usize, 
    false_monke: usize,
    inspections: f64,
}

//monke has an implementation
impl Monke {
    //constructor
    fn new_monke(items: Vec::<f64>,operation: char, operation_value: f64,divisor: f64,true_monke: usize,false_monke: usize) -> Self{
        Self{
            items: items,
            operation: operation,
            operation_value: operation_value,
            divisor: divisor,
            true_monke: true_monke, 
            false_monke: false_monke,
            inspections: 0.0,
        }
    }

    //do a monke, return the monkey nummer and item value 
    fn process(&mut self) -> (usize,f64) {
        //the item's worry level is changed
        let mut tmp_item = self.items[0];
        if self.operation == '+'{
            if self.operation_value == 0.0{
                tmp_item = tmp_item + tmp_item;
            }else{
                 tmp_item = tmp_item + self.operation_value;
            }
        }
        else if self.operation == '*'{
            if self.operation_value == 0.0{
                tmp_item = tmp_item * tmp_item;
            }else{
                tmp_item = tmp_item * self.operation_value;
            }
        }else{
            panic!("bad operation")
        }
        //monke bored, divide by 3, round down (automatic for i32)
        tmp_item = ( tmp_item/3.0 ).floor(); //<--- uncomment for part 1
        //test to which monkey we should throw
        //becomes funny wihth 64 .. test if it is very close to whole number
        if tmp_item % self.divisor < 0.01{
            //truemonke
            //monkey ditched the item! pop first
            self.items.remove(0);
            self.inspections +=1.0;
            return(self.true_monke, tmp_item)
        }
        else{
            // falsemonke
            //monkey ditched the item! pop first
            self.items.remove(0);
            self.inspections +=1.0;
            return(self.false_monke, tmp_item)
        }

    }
}




//we have a list of monkes
//monke_list = Vec<Monke>:new();


fn main() {
    //build example monkes
    let monke0 = Monke::new_monke(vec![79.0,98.0],'*',19.0,23.0,2,3);
    let monke1 = Monke::new_monke(vec![54.0, 65.0, 75.0, 74.0],'+',6.0,19.0,2,0);
    let monke2 = Monke::new_monke(vec![79.0, 60.0, 97.0],'*',0.0,13.0,1,3); //old represented by 0
    let monke3 = Monke::new_monke(vec![74.0],'+',3.0,17.0,0,1);
    let mut monkevec = vec![monke0,monke1,monke2,monke3];

    //build real input monkeys
    //let mut monke0 = Monke::new_monke(vec![56.0, 52.0, 58.0, 96.0, 70.0, 75.0, 72.0],'*',17.0,11.0,2,3);
    //let mut monke1 = Monke::new_monke(vec![75.0, 58.0, 86.0, 80.0, 55.0, 81.0],'+',7.0,3.0,6,5);
    //let mut monke2 = Monke::new_monke(vec![73.0, 68.0, 73.0, 90.0],'*',0.0,5.0,1,7);
    //let mut monke3 = Monke::new_monke(vec![72.0, 89.0, 55.0, 51.0, 59.0],'+',1.0,7.0,2,7);
    //let mut monke4 = Monke::new_monke(vec![76.0, 76.0, 91.0],'*',3.0,19.0,0,3);
    //let mut monke5 = Monke::new_monke(vec![88.0],'+',4.0,2.0,6,4);
    //let mut monke6 = Monke::new_monke(vec![64.0, 63.0, 56.0, 50.0, 77.0, 55.0, 55.0, 86.0],'+',8.0,13.0,4,0);
    //let mut monke7 = Monke::new_monke(vec![79.0, 58.0],'+',6.0,17.0,1,5);
    //let mut monkevec = vec![monke0,monke1,monke2,monke3,monke4,monke5,monke6,monke7];

    let iterations: i32 = 10000;//<--20 for part 1
    for _i in 0..iterations{
        for monke_now in 0..monkevec.len(){
            //monke throws away all items each time
            while monkevec[monke_now].items.len() > 0{
                let (monke_to, monke_item) = monkevec[monke_now].process();
                monkevec[monke_to].items.push(monke_item);
            }
        }
       // for monke_now in 0..monkevec.len(){
       //     //print holdings after each rount
       //     println!{"Monke {:} holding {:?}",monke_now,monkevec[monke_now].items}
       //}
       // println!("________________")
    }

   //get the highest two monke scores to ge the monke business
   let mut highest = 0.0;
   let mut second_to_highest = 0.0;

   for monke_now in 0..monkevec.len(){
       if monkevec[monke_now].inspections > highest{
           second_to_highest = highest;
           highest = monkevec[monke_now].inspections;
       }

       //println!("monke {:} inspectec {:} times",monke_now,monkevec[monke_now].inspections)
   }
   println!("highest monke: {:?}",highest);
   println!("2nd to highest monke: {:?}",second_to_highest);
   println!("multiplied: {:?}",second_to_highest*highest);
}
