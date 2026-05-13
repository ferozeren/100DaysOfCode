fn main() {
    let _ = [0_i32; 100]; // Best
                          // Or
    let _: [i32; 100] = [0; 100]; //  Use when the size/type is critical for safety.
                                  // Or
    let _: [i32; _] = [0; 100]; // Ok if you dont' care about size of array, again wihtout annotaion would be better
                                // Or
    let a: [_; _] = [0_i32; 100]; // Better use without annotation

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
