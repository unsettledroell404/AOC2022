use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;
 use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Self> {
        let mut result = Vec::new();

        // up
        if self.y > 0 {
            result.push(Self {
                x: self.x,
                y: self.y - 1,
            });
        }
        // down
        if self.y < rows - 1 {
            result.push(Self {
                x: self.x,
                y: self.y + 1,
            });
        }
        // left
        if self.x > 0 {
            result.push(Self {
                x: self.x - 1,
                y: self.y,
            });
        }
        // right
        if self.x < cols - 1 {
            result.push(Self {
                x: self.x + 1,
                y: self.y,
            });
        }
        result
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Node {
    cost: u32,
    coord: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut grid :Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(s) = line{
               grid.push(s.chars().collect());
            }
        }
    }
    print_grid(&grid);

    //let's find the position of E and S first (part could be wrong)
    let grid_len = grid[0].len();//length of a row
    let start_pos_flat = grid.iter().flatten().position(|element| *element=='S').unwrap();  //outer vec(cols) then inner (rows)
    let start_pos = (start_pos_flat%(grid_len),start_pos_flat/grid_len);                    //(row,col)
    let end_pos_flat = grid.iter().flatten().position(|element| *element=='E').unwrap();
    let end_pos = (end_pos_flat%grid_len,end_pos_flat/grid_len);                            //(row,col)
    println!("Start pos: {:?}, end pos: {:?}",start_pos,end_pos);

    let grid_size = (grid[0].len(),grid.len());

    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    
    //part 1
    grid[start_pos.1][start_pos.0] = 'a';
    grid[end_pos.1][end_pos.0] = 'z';
    let mut start = Coord { x: start_pos.0, y:start_pos.1 };
    let mut end = Coord { x: end_pos.0, y: end_pos.1 };

    pq.push(Node {
        cost: 0,
        coord: end,
    });
    visited.insert(start);



    while let Some(Node { coord, cost }) = pq.pop() {
        //p1
        //if coord == end {
        //    println!("costs:{:}",cost);
        //    return
        //}

        let curr_height = grid[coord.y][coord.x];

        //p2
        if curr_height == 'a'{
            println!("costs:{:}",cost);    
            return;
        }

        let neighbours = coord.neighbours(grid_size.1, grid_size.0);
        let candidates: Vec<_> = neighbours
            .iter()
            .filter(|coord| {
                let height = grid[coord.y][coord.x];
                //p1 height <= curr_height || height == ((curr_height as u8) + 1) as char
                height >= curr_height || height == ((curr_height as u8) - 1) as char
            })
            .collect();

        for candidate in candidates {
            if visited.insert(*candidate) {
                pq.push(Node {
                    cost: cost + 1,
                    coord: *candidate,
                })
            }
        }
    } 
}

fn print_grid (grid :&Vec<Vec<char>>) -> (){
    for i in 0..grid.len(){
        println!("{:?}",&grid[i].iter().collect::<String>());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
