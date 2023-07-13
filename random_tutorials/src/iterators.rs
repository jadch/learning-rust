pub fn iterate() {
    println!("iterate");
    basic_iteration();
}

fn basic_iteration() {
    let vec = vec![1, 2, 3, 4];
    let mut vec_iter = vec.iter(); // Iterator can mutate self

    let one: Option<&i32> = vec_iter.next();
    println!("num one: {:?}", one.unwrap());

    for num in vec_iter {
        // starts at 2
        println!("num: {} ", num);
    }
}