use rand::{thread_rng, Rng};
use std::ops::{Div, Mul};

pub struct RubinMillerTest {
    // we only need an input of the number
    // to test for (num) and the number of
    // cases (k)
    pub num: usize,
    pub k: usize,
    pub r: usize,
    pub d: usize,
}

impl RubinMillerTest {
    // set up test with inputs
    pub fn new(num: usize, k: usize) -> RubinMillerTest {
        RubinMillerTest { num, k, r: 0, d: 0 }
    }
    // this will get our common tests out
    // of the way
    pub fn preliminary_tests(&self) -> bool {
        if self.num == 2 || self.num == 3 {
            return true;
        } else if self.num == 1 || self.num % 2 == 0 {
            return false;
        } else {
            return true;
        }
    }

    // factors out powers of 2
    pub fn upper_range(&mut self) {
        self.d = self.num - 1;
        while self.d % 2 == 0 {
            self.d = self.d.div(2) as usize;
            self.r += 1;
        }
    }

    pub fn test(&mut self) -> bool {
        //random number genration
        let mut rng = thread_rng();

        for _i in 0..self.k {
            let a = rng.gen_range(2..self.num - 2);
            let mut x = (a.pow(self.d as u32)) % self.num;
            if x == 1 || x == self.num - 1 {
                continue;
            }
            for _j in 0..self.r - 1 {
                x = x.pow(2) % self.num;
                if x == self.num - 1 {
                    break;
                }
            }
            if x != self.num-1 {
                return false;
            }
        }
        true
    }
}
