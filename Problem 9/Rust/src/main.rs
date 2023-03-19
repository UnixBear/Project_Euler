mod pythagtrip;
use pythagtrip::Triangle;

fn main() {
    let n = 1000;
    let mut triplets = Vec::new();
    for a in 1..n-1 {
        for b in a..n-a {
            let c = n - a - b;
            let triplet = Triangle::new(a,b,c);
            triplets.push(triplet)
        }
    }
    for triplet in triplets.iter() {
        if triplet.triplechecker() && triplet.sum_check(1000) {
            println!("{}", triplet);
            println!("and the product is: {}", triplet.product());
            return;
        }
    }
}
