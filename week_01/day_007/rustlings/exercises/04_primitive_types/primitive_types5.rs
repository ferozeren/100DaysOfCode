fn main() {
    let cat = ("Furry McFurson", 3.5);

    let (name, age): (&str, f64) = cat; //  Destructure the `cat` tuple in one statement
                                        // Or
                                        //let (name, age) = (cat.0, cat.1);

    println!("{name} is {age} years old");
}
