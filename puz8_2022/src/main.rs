use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate array2d;
use array2d::Array2D;

fn main() {
    //Check if file exists
    if let Ok(lines) = read_lines("./input.txt") {
        //build the entire matrix
        let mut nested_vector:Vec<Vec<u8>> = Vec::new();
        let mut linenum:usize = 0;
        for line in lines {
            if let Ok(s) = line {
            let intvec: Vec<u8> = s.as_bytes().to_vec().iter().map(|c| c-48).collect();    
            nested_vector.push(intvec.clone());
            }
            linenum+=1;
        }

        let array = Array2D::from_rows(&nested_vector);

        //part 1
        let mut numhidden:usize = 0;
        let totaltrees:usize = array.num_rows()*array.num_columns();

        //iterating over the array from 1:99
        for rownum in 1..array.num_rows()-1{
            for colnum in 1..array.num_columns()-1{
                let currentTree = array[(rownum,colnum)];
                //check trees to the left: are any higher or same?    
                let lefthigher  = array.row_iter(rownum).enumerate().any(|(i,c)| c>=&currentTree && i<colnum);
                let righthigher = array.row_iter(rownum).enumerate().any(|(i,c)| c>=&currentTree && i>colnum);
                //check trees to the left and right: are any higher or same?
                let tophigher    = array.column_iter(colnum).enumerate().any(|(i,c)| c>=&currentTree && i<rownum);
                let bottomhigher = array.column_iter(colnum).enumerate().any(|(i,c)| c>=&currentTree && i>rownum);

                if(lefthigher&&righthigher&&tophigher&&bottomhigher){
                    numhidden+=1;
                }
            }
        
        }
        println!("num hidden trees = {:?}",numhidden);
        println!("total number of trees = {:?}",totaltrees);
        println!("number visible trees = {:?}",totaltrees-numhidden);

        //part 2
        let mut highestTotalScore: i32 = 0;
        for rownum in 1..array.num_rows()-1{
            for colnum in 1..array.num_columns()-1{
                let currentTree = array[(rownum,colnum)];
                //check trees to the left: are any higher or same? What is the position?
                let left_to_right    =  find_score(array.row_iter(rownum)   .enumerate().position(|(i,c)| c>=&currentTree && i>colnum),colnum,array.num_columns()-colnum-1);
                let right_to_left    =  find_score(array.row_iter(rownum)   .enumerate().position(|(i,c)| c>=&currentTree && i<colnum),colnum,colnum);//this iterates the wrong way - rev not implemented
                let top_to_bottom    =  find_score(array.column_iter(colnum).enumerate().position(|(i,c)| c>=&currentTree && i>rownum),rownum,array.num_rows()-rownum-1);
                let bottom_to_top    =  find_score(array.column_iter(colnum).enumerate().position(|(i,c)| c>=&currentTree && i<rownum),rownum,rownum);//this iterates the wrong way - rev not implemented
                let totalscore = left_to_right*right_to_left*top_to_bottom*bottom_to_top;
                if(totalscore>highestTotalScore){
                    highestTotalScore=totalscore;
                }
            }
        }
        println!("Incorrect: Highest scenic view = {:?}",highestTotalScore);

        //part 2 simpler method:
        let mut highest_scenic_view: i32 = 0;
        for rownum in 1..array.num_rows()-1{
            for colnum in 1..array.num_columns()-1{
                let mut scenic_east: i32 = 0;
                let mut scenic_west: i32 = 0;
                let mut scenic_north: i32 = 0;
                let mut scenic_south: i32 = 0;

                //check walking east
                for col_other in colnum+1..array.num_columns(){
                    scenic_east+=1;
                    if array[(rownum,col_other)]>=array[(rownum,colnum)]{
                        break;
                    }
                }
                //check walking west
                for col_other in (0..=colnum-1).rev(){
                    scenic_west+=1;
                    if array[(rownum,col_other)]>=array[(rownum,colnum)]{
                        break;
                    }
                }
                //check walking south
                for row_other in rownum+1..array.num_rows(){
                    scenic_south+=1;
                    if array[(row_other,colnum)]>=array[(rownum,colnum)]{
                        break;
                    }
                }
                //check walking north
                for row_other in (0..=rownum-1).rev(){
                    scenic_north+=1;
                    if array[(row_other,colnum)]>=array[(rownum,colnum)]{
                        break;
                    }
                }
                let scenic_view = scenic_south*scenic_north*scenic_east*scenic_west;
                if highest_scenic_view<scenic_view{
                    highest_scenic_view=scenic_view;
                }
            }
        }
        println!("Correct scenic view score: {:}",highest_scenic_view);
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
fn find_score(x: Option<usize>, currentind: usize, otherwise: usize) -> i32{
// Pattern match to retrieve the value
    match x {
        Some(x) => (x as i32-currentind as i32).abs(),
        None    => (otherwise as i32),
     }
}