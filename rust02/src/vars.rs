pub fn run() {
    // Variables are immutable by default in Rust
    let name = "Martin";
    println!("My name is {}", name);
    
    const ID: i32 = 001;
    // I can assign multiple vars using this kind of 'destructuring'
    let (var1, var2) = ("string", 3);
}
