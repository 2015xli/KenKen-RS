use crate::problem::*;

impl Region{
    fn match_add(&self, cand: &Candidate) -> bool {
        let mut sum = 0;
        for p in &self.points {
            sum += cand[p.x][p.y];
        }

        sum == self.val
    }

    fn match_sub(&self, cand: &Candidate) -> bool {
        let p0 = &self.points[0];
        let p1 = &self.points[1];
        let v0 = cand[p0.x][p0.y];        
        let v1 = cand[p1.x][p1.y];        

        if v1 >= v0 { (v1-v0) == self.val }
        else { (v0-v1) == self.val }
        
    }

    fn match_mul(&self, cand: &Candidate) -> bool {
        let mut prod = 1;
        for p in &self.points {
            prod *= cand[p.x][p.y];
        }

        prod == self.val
    }

    fn match_div(&self, cand: &Candidate) -> bool {
        let p0 = &self.points[0];
        let p1 = &self.points[1];
        let v0 = cand[p0.x][p0.y];        
        let v1 = cand[p1.x][p1.y];        

        (v1/v0) == self.val || (v0/v1) == self.val        
    }

    fn match_nop(&self, cand: &Candidate) -> bool {
        let p = &self.points[0];

        cand[p.x][p.y] == self.val
    }

    fn match_cand(&self, cand:&Candidate) -> bool {
        match self.op {
            Op::ADD => self.match_add(cand),
            Op::SUB => self.match_sub(cand),
            Op::MUL => self.match_mul(cand),
            Op::DIV => self.match_div(cand),
            Op::NOP => self.match_nop(cand),
        }
    }
}

#[derive(Debug, Clone)]
pub struct State{
    pub col_avails: Vec<Vec<usize>>,
    pub row_avails: Vec<Vec<usize>>,
}

impl State{
    pub fn new(dim: usize) -> Self {
        let row_avails = vec![vec![1; dim]; dim];
        let col_avails = row_avails.to_vec();
        State{
            col_avails,
            row_avails,
        }
    }
}


type Candidate = Vec<Vec<usize>>;

fn output(cand: &Candidate){
    for row in cand{
        println!("{:?}", row);
    }
    println!("");
}


pub struct Solution{
    problem: Problem,
    cand: Candidate,
    sols: Vec<Candidate>,
}

impl Solution {

    pub fn new(problem: Problem) -> Self {
        let dim = problem.dim;
        Solution{
            problem,
            cand: vec![vec![0; dim]; dim],
            sols: Vec::new(),
        }
    }

    pub fn find(&mut self, x:usize, y:usize, state: &mut State){                
        //find all the available values for position (x,y)
        let dim = self.problem.dim;
        let mut avails = vec![0; dim];
        for i in 0..dim {
            if state.row_avails[x][i] == 1 && state.col_avails[y][i] == 1 {
                avails[i] = 1;   //1 means value i+1 is available
            } 
        }

        for i in 0..dim {
            self.cand[x][y] = i + 1; 

            if avails[i] == 0 { continue; }

            if let Some(region) = &self.problem.matchset[x][y] {
                if ! region.match_cand(&self.cand) { 
                    continue; 
                }
                //output(&self.cand);
            }            

            state.row_avails[x][i] = 0;  //value i+1 is unavailable now
            state.col_avails[y][i] = 0;

            if x+1 != dim && y+1 == dim {
                self.find(x+1, 0, state); //wrap around to next row
            }else if y+1 != dim {
                self.find(x, y+1, state); //move to next col
            }else{  //(x+1,y+1) = (dim, dim), i.e. last position
                self.sols.push(self.cand.clone());
            }           

            state.row_avails[x][i] = 1;  //value i+1 is available now
            state.col_avails[y][i] = 1;

        }
    }

    pub fn output(&self) {
        let sols = &self.sols;
        println!("\nFind {} solutions:\n", sols.len());
        for sol in sols {
            output(sol);
        }
    }
}
