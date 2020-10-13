pub fn guess() {
    println!("Guess examples  ");
    println!("----------------");

    // Read from stdin
    let mut line = String::from("");
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to load line");
    println!("Your input: {}", line);
    let heart_eyed_cat = '😻';
    println!("Cat? {}", heart_eyed_cat);

    let a = [3; 5]; // array of 5 elements each equal to 3 !
    let b: [i32; 5] = [1, 2, 3, 4, 5]; //initialize static array
    println!("{} + {} = {}", a[0], a[1], b[1] + b[3]);

    // enumerate with index
    for (i, e) in b.iter().enumerate() {
        println!("b[{}] = {}", i, e);
    }
}
