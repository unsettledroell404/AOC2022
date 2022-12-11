use std::collections::BinaryHeap;

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
    fn new_monke(items: Vec::<i64>,operation: char, operation_value: i64,divisor: i64,true_monke: usize,false_monke: usize) -> Self{
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
        //the item's worry level is changed
        let mut tmp_item = self.items[0];
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
        if tmp_item % self.divisor == 0{
            //truemonke
            //monke ditched the item! pop first
            self.items.remove(0);
            self.inspections +=1;
            return(self.true_monke, tmp_item)
        }
        else{
            //falsemonke
            //monke ditched the item! pop first
            self.items.remove(0);
            self.inspections +=1;
            return(self.false_monke, tmp_item)
        }

    }
}

fn main() {
    //build example monkes
    //let monke0 = Monke::new_monke(vec![79,98],'*',19,23,2,3);
    //let monke1 = Monke::new_monke(vec![54, 65, 75, 74],'+',6,19,2,0);
    //let monke2 = Monke::new_monke(vec![79, 60, 97],'*',0,13,1,3); //old represented by 0
    //let monke3 = Monke::new_monke(vec![74],'+',3,17,0,1);
    //let mut monkevec = vec![monke0,monke1,monke2,monke3];

    //do a trick to keep the worry low for part 2
    //build real input monkeys
    let mut monke0 = Monke::new_monke(vec![56, 52, 58, 96, 70, 75, 72],'*',17,11,2,3);
    let mut monke1 = Monke::new_monke(vec![75, 58, 86, 80, 55, 81],'+',7,3,6,5);
    let mut monke2 = Monke::new_monke(vec![73, 68, 73, 90],'*',0,5,1,7);
    let mut monke3 = Monke::new_monke(vec![72, 89, 55, 51, 59],'+',1,7,2,7);
    let mut monke4 = Monke::new_monke(vec![76, 76, 91],'*',3,19,0,3);
    let mut monke5 = Monke::new_monke(vec![88],'+',4,2,6,4);
    let mut monke6 = Monke::new_monke(vec![64, 63, 56, 50, 77, 55, 55, 86],'+',8,13,4,0);
    let mut monke7 = Monke::new_monke(vec![79, 58],'+',6,17,1,5);
    let mut monkevec = vec![monke0,monke1,monke2,monke3,monke4,monke5,monke6,monke7];

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
}
