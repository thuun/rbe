fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!("Base 10:               {}", 69420);
    println!("Base 2 (binary):       {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hex lower):   {:x}", 69420);
    println!("Base 16 (hex upper):   {:X}", 69420);
    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);
    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
    let pi = 3.141592;
    println!("{pi:.3}");
}
