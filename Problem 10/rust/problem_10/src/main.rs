
mod rubin_miller_test;
use rubin_miller_test::RubinMillerTest;

fn main() {
    let mut test = RubinMillerTest::new(10,20);
    let temp = test.test();
    println!("{}",temp);
}
