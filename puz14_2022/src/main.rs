//use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs;

#[derive(Debug, Default)]
struct Grid{
	grid: Vec<Vec<char>>,
	rocks: Vec<Rock>,
	xmin: usize,
	xmax: usize,
	ymin: usize,
	ymax: usize,
}

#[derive(Debug, Default, Clone)]
struct Rock{
	locations: Vec<Vec<usize>>,
}

impl Rock{
	fn new_Rock(corner_points: Vec<Vec<usize>>) -> Rock{
		//vec of vecs -> Npoints * 2
		let mut locations: Vec<Vec<usize>> = Vec::new();
		let mut corner_iter = corner_points.iter().peekable();
		while let Some(corner) = corner_iter.next()
		{
			//println!("corner: {:?}",corner);
			//peek to see the next corner. If it is there, draw the line
			if let Some(peeked_corner) = corner_iter.peek(){
				//println!("peeked corner: {:?}",peeked_corner);
				//draw a line between the points.
				if corner[0]==peeked_corner[0]{
					//hline
					if peeked_corner[1]>corner[1]{
						for i in corner[1]..=peeked_corner[1]
						{
							locations.push([corner[0],i].to_vec());
						}
					}else{
						for i in peeked_corner[1]..=corner[1]
						{
							locations.push([corner[0],i].to_vec());
						}
					}
				}
				else if  corner[1]==peeked_corner[1]{
					//vline
					if peeked_corner[0]>corner[0]{
						for i in corner[0]..=peeked_corner[0]
						{
							locations.push([i,corner[1]].to_vec());
						}
					}else{
						for i in peeked_corner[0]..=corner[0]
						{
							locations.push([i,corner[1]].to_vec());
						}
					}
				}else{
					panic!("Lines are not horizontal nor vertical, should not happen!")
				}
				//println!("locations: {:?}",locations);	
			}
		}
		return Rock{locations};
	}
}

impl Grid{
	fn new_grid(input: &str) -> Self{
		let mut grid = Grid{..Default::default()};	
		let mut xmin :usize= 1000;
		let ymin :usize= 0; //ymin is always 0
		let mut xmax :usize= 0;
		let mut ymax :usize = 0;
		let lines: Vec<&str> = input.split("\n").collect();
			for line in lines{
				println!("{:?}",&line);
			//iterate through to find min and max of x and y
			//collect the lines in a vector
			//let binding =  INPUT.replace("->",",");
			let mut new_rock_vector: Vec<Vec<usize>> = Vec::new();
			let binding =  line.replace(" ","");
			let rock_vector :Vec<&str> = binding.split("->").collect();//split to points
			for rock in &rock_vector{
				let rock2=rock.clone();
				let xy :Vec<&str> = rock2.split(",").collect();
				let xy :Vec<usize> = xy.iter().map(|c| c.parse().unwrap()).collect();
				// log the sizes all the time to find the min and max x and y to use
				if xy[0]>xmax{xmax = xy[0];}
				if xy[0]<xmin{xmin = xy[0];}
				if xy[1]>ymax{ymax = xy[1];}
				println!("{:?}",xy);
				new_rock_vector.push(xy);
			}
			let new_rock = Rock::new_Rock(new_rock_vector);
			// add the rocks to the grid
			grid.rocks.push(new_rock);
		}

		//add the floor part 2
		ymax = ymax+2;
		xmin = 0;//massive floor plox
		xmax = 1000;
		let veccie: Vec<Vec<usize>> = Vec::from([Vec::from([xmin,ymax]),Vec::from([xmax,ymax])]);
		let new_rock = Rock::new_Rock(veccie);//x1y1-x2y2
		grid.rocks.push(new_rock);
		

		//add the rest of the grid information
		grid.xmax = xmax;grid.xmin = xmin;grid.ymax = ymax;grid.ymin = ymin;
		//paint the grid with "."
		for y in ymin..=ymax{//iterate over the rows
				grid.grid.push(vec!('.';(xmax-xmin+1)));
		}
		//add rocks
		for rock in grid.rocks.clone(){
			//iterate over the location
			for location in rock.locations{
				 grid.grid[location[1]-grid.ymin][location[0]-grid.xmin] = '#';
			}
		} 

		return grid;
	}
	fn print_grid(&self) -> (){
		for i in 0..=(self.ymax-self.ymin){
				println!("{:?}",self.grid[i]);
		}
	}
	fn drop_sand(&mut self,sandcoorx:usize,sandcoory:usize) -> bool{
		//find first position where the grid is occupied (not .)
		let mut row_dropped = 0;
		for rowind in sandcoory..self.grid.len() {
			//check if we will overflow

			//drop in place
			if self.grid[rowind][sandcoorx-self.xmin] != '.' && self.grid[rowind][sandcoorx-self.xmin-1] != '.' && self.grid[rowind][sandcoorx-self.xmin+1] != '.'{
				row_dropped = rowind;
				break;
			}
			//drop left
			else if self.grid[rowind][sandcoorx-self.xmin] != '.' && self.grid[rowind][sandcoorx-self.xmin-1] == '.'{
				let b = self.drop_sand(sandcoorx-1,rowind);
				return b;
			}
			//drop right
			else if self.grid[rowind][sandcoorx-self.xmin] != '.' && self.grid[rowind][sandcoorx-self.xmin-1] != '.' && self.grid[rowind][sandcoorx-self.xmin+1] == '.'{
				let b = self.drop_sand(sandcoorx+1,rowind);
				return b;
			}
			else{
				//self.drop_sand(sandcoorx,0);
				//do iteration according to the rules
			}
		}
		self.grid[row_dropped-1][sandcoorx-self.xmin]='o';//-1 because it drops on top
		if row_dropped == 1{
			return true;
		}
		return false
	}
}

fn main() {
let file = fs::read_to_string("./input.txt").unwrap().replace("\r","");
	let mut grid = Grid::new_grid(&file);
	//grid.print_grid();
	//sand source is 500,0
	for i in 1..=100000{
	if grid.drop_sand(500,0) != true{
		//it panics there as soon as it overflows .. lol
			println!("{:} succesful drops",i);
		}
		else{
			println!("dropped at source at {:}, ending ..  ",i);
			break;
		}
	}
	//grid.print_grid();
}

