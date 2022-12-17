//use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate nom;
use nom::branch::alt;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::character::complete::newline;
use nom::sequence::separated_pair;
use std::cmp::Ordering::*;
use nom::lib::std::cmp::Ordering;
use std::fs;

#[derive(Debug,Eq, Ord, PartialEq, PartialOrd)]
struct Pair{
    left: Packet,
    right: Packet,
}
#[derive(Debug,Eq)]
enum Packet{
    List(Vec<Packet>),
    Number(u32),
}

//i literally copied the stuff below here
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => {
                l0 == r0
            }
            (Self::List(l0), Self::Number(r0)) => { //to wrap the number in a list so that it can be compared
                l0 == &vec![Packet::Number(*r0)]
            }
            (Self::Number(l0), Self::List(r0)) => { //same as above
                &vec![Packet::Number(*l0)] == r0
            }
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b), //this is fine
            (Packet::List(a), Packet::Number(b)) => {       //number to list, then compare
                a.cmp(&vec![Packet::Number(*b)])
            }
            (Packet::Number(a), Packet::List(b)) => {       //same as above
                vec![Packet::Number(*a)].cmp(&b)
            }
            (Packet::Number(a), Packet::Number(b)) => {     //this is fine
                a.cmp(b)
            }
        }
    }
}


fn packet(input: &str) -> IResult<&str, Packet>{    //take an incoming string and turn it into a packet, which is either a number or a list
    alt((                                           //alt returns the result of the first succesful parser
        delimited(                                  //parser: if delimited by [ and ]
            tag("["),
            separated_list0(tag(","),packet),       //this is recursive
            tag("]"),
         ).map(|vec| Packet::List(vec)),            //map over each element in that vector and turn it into a packet::list
         nom::character::complete::u32              //parser: recognizes a u32
         .map(|num| Packet::Number(num)),           //iterate over each number and turn it into a packet::number
         ))(input)                                  //<--call this parser on input
}

fn pairs(input: &str) -> IResult<&str, Vec<Pair>>{
    separated_list1(                                //alternates between 2 parsers
        tag("\n\n"),
        separated_pair(packet,newline,packet).map(
        |(packet1, packet2)| Pair {
            left: packet1,
            right: packet2,
        },
        ),
    )(input)
}

fn main() {
let file = fs::read_to_string("./input_test.txt").unwrap().replace("\r","");
println!("correct indexes (test): {:}",part1(&file));
let file = fs::read_to_string("./input.txt").unwrap().replace("\r","");
println!("correct indexes (real): {:}", part1(&file));
}

pub fn part1(input: &str) -> String{
    let (_,pair_list) = pairs(input).unwrap();
    //iterate over both lists
    pair_list.iter().enumerate().filter_map(
    |(i, Pair {left,right})| match left.cmp(right) 
    {
        Less => Some(i),//return index if there is some i
        Equal => panic!("should never be equal"),
        Greater => None,//do nothing

    }) //need to derive partial comp for this
    .map(|v| v+1) //to get the indexes correct
    .sum::<usize>().to_string()  //sum the indexes 
}
