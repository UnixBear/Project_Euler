use core::fmt;

pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Triangle {
    pub fn new(a: usize, b: usize, c: usize) -> Triangle {
        Triangle { a, b, c }
    }
    pub fn triplechecker(&self) -> bool {
        let formula = ((self.a * self.a) + (self.b * self.b)) == (self.c * self.c);
        if formula {
            true
        } else {
            false
        }
    }

    pub fn sum_check(&self, n: usize) -> bool {
        self.a + self.b + self.c == n
    }

    pub fn product(&self) -> usize {
        self.a * self.b * self.c
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.a, self.b, self.c)
    }
}
