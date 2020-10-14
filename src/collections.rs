use std::collections::HashMap;

pub fn collect() {
    println!("Collections :)");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 120);
    scores.insert(String::from("White"), 255);

    let s = "Ala ma Blue cat oraz Yellow shirt, don't mix with Blue";
    for w in s.split_whitespace() {
        let e = scores.entry(w.to_string()).or_insert(0);
        *e += 1;

    }

    println!("Mapa: {:?}", scores);
}
