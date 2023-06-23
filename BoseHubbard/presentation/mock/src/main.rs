fn main() {
    let hello_world = String::from("Hello ");

    append_world(hello_world);

    println!("{hello_world}");
}

fn append_world(input: String) {
    input.push_str("World!");
}
