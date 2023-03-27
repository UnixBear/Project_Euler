use std::ops::Div;
use rand::Rng;


pub struct RubinMillerTest {
    // we only need an input of the number
    // to test for (num) and the number of 
    // cases (k)
    pub num:usize,
    pub k:usize,
    pub r:usize,
}

impl RubinMillerTest {
    // set up test with inputs
    pub fn new(num:usize, k:usize) -> RubinMillerTest {
        RubinMillerTest { num, k, r: 0 }
    }
    // this will get our common tests out
    // of the way
    pub fn preliminary_tests(&self)->bool {
        if self.num == 2 || self.num == 3 {
            return true;
        }
        else if self.num == 1 || self.num % 2 ==0 {
            return false;
        }
        else {
            return true;
        }
    }

    // factors out powers of 2
    pub fn upper_range(&mut self) {
        let mut d = self.num -1;
        while d % 2 == 0 {
            d = d.div(2) as usize;
            self.r += 1;
        }
    }

    pub fn test(&mut self) {
        for i in 0..self.r {
            
        }
    }
}