
#[derive(Debug, Clone)]
pub struct Point{ 
    pub x:usize, 
    pub y:usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op{ ADD, SUB, MUL, DIV, NOP }

#[derive(Debug, Clone)]
pub struct Region{
    pub op: Op,
    pub val: usize,
    pub points: Vec<Point>
}

#[derive(Debug, Clone)]
pub struct Problem{
    pub regions: Vec<Region>,
    pub dim: usize, 
    pub matchset: Vec<Vec<Option<Region>>>,
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

impl Problem{
    pub fn new(file: &String) -> Self {
        let f = File::open(file).expect("Cannot open input file!");
        let mut input = BufReader::new(&f);
        let mut map: HashMap<char, Vec<Point>> = HashMap::new();
        let mut x=0; 
        let mut y; 
        let mut n_col=0;
        for l in (&mut input).lines(){
            let line = l.unwrap();
            if line.is_empty() { break; }
            println!("{}", line);

            let chars = line.split_whitespace();
            y = 0;
            for chs in chars{
                if chs.len() != 1 { panic!("The position has more than one char: {:?}!", chs); }
                let ch = chs.chars().next().unwrap();
                map.entry(ch).or_insert_with(Vec::new).push(Point{x, y}); 
                y += 1;
            }            

            if n_col != 0 { assert_eq!(n_col, y, "Input matrix is not aligned!"); }
            else { n_col = y; }
            x += 1;
        }
        assert_eq!(x, n_col, "Input matrix is incorrect."); 
        println!("");
        //println!("{:?}", map);
        let mut regions = Vec::new();
        for l in input.lines(){
            let line = l.unwrap();
            if line.is_empty() { break; }
            println!("{}", line);

            let mut chars = line.split_whitespace();

            let chs = chars.next().unwrap();
            let ch = chs.chars().next().unwrap();
            let points = map.get(&ch).expect("The region symbol does not exist in the matrix.");

            let chs = chars.next().unwrap();
            let val = chs.parse::<usize>().expect("The region does not have a correct value.");

            let op;
            match chars.next() {
                None => op = Op::NOP,
                Some(chs) =>{
                    let ch = chs.chars().next().unwrap();
                    op = match ch{
                        '+' => Op::ADD,
                        '-' => Op::SUB,
                        'X'|'x'|'*' => Op::MUL,
                        '/' => Op::DIV,
                        _ => panic!("The region has incorrect operator."),
                    }
                }
            }
            if ( op == Op::NOP && points.len() != 1 ) 
                || ( op != Op::NOP && points.len() == 1 ) 
                    { panic!("Incorrect region definition.") }

            regions.push(Region{op, val, points:points.to_vec()});
        }

        //println!("{:?}", regions);

        let matchset = Problem::opt_problem(&regions, n_col);

        Problem{
            regions, 
            dim: n_col,
            matchset
        }
    }

    pub fn opt_problem(regions: &Vec<Region>, dim: usize) -> Vec<Vec<Option<Region>>>{

        let mut matchset: Vec<Vec<Option<Region>>> = vec![vec![None; dim]; dim];
        
        for region in regions {
            let mut max_p = &region.points[0];
            for p in &region.points {
                if max_p.x < p.x  ||
                    (max_p.x == p.x && max_p.y < p.y){ 
                    max_p = p;
                }
            }
            matchset[max_p.x][max_p.y] = Some(region.clone());
        }

        matchset
    }
}
