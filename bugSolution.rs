fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Check if the index is within the bounds of the vector before accessing the element
    if vec.len() > 3 {
        println!("{}", vec[3]);
    } else {
        println!("Index out of bounds");
    }
} 
//Alternatively, you can use the get() method which returns an Option
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    match vec.get(3) {
        Some(value) => println!("{}", value),
        None => println!("Index out of bounds"),
    }
}