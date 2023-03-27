mod rubin_miller_test;
use rubin_miller_test::RubinMillerTest;

fn main() {
    let test = RubinMillerTest::new(2,20);
    let temp = test.preliminary_tests();
    println!("{}",temp);
}
