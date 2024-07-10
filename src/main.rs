mod rot13;

fn main() {
    // better_panic::Settings::new()
    //     .lineno_suffix(true)
    //     .message("Timeless you fucking suck, the program has crashed")
    //     .verbosity(better_panic::Verbosity::Full)
    //     .install();
    let start_of_time = std::time::Instant::now();
    let a = rot13::Rot13Encryption::encrypt("Hello world!");
    println!(
        "Took (Single threaded): {} microseconds",
        start_of_time.elapsed().as_micros()
    );
    assert_eq!(a, "Uryyb jbeyq!");
    assert_eq!(rot13::Rot13Encryption::decrypt(&a), "Hello world!");
}
