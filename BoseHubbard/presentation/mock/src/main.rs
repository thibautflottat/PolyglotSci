fn main() {
    let mut hello_world = String::from("Hello ");

    append_world(&mut hello_world);

    println!("{hello_world}");
}

fn append_world(input: &mut String) {
    input.push_str("World!");
}
