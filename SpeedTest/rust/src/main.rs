use std::time::Instant;
fn main() {
    let before = Instant::now();

    for i in 0..5000 {
        for j in 0..5000 {}
    }
    println!("Elapsed time: {:.2?}", before.elapsed());
}