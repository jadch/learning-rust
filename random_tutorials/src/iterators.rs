pub fn iterate() {
    println!("iterate");
    iterator_methods();
}

#[allow(dead_code)]
fn basic_iteration() {
    let vec = vec![1, 2, 3, 4];
    let mut vec_iter = vec.iter(); // Iterator "next" method can mutate self

    let one: Option<&i32> = vec_iter.next();
    println!("num one: {:?}", one.unwrap());

    for num in vec_iter {
        println!("num: {} ", num); // starts at 2
    }
}

fn iterator_methods() {
    let vec = vec![1, 2, 3, 4];
    let sum: i32 = vec.iter().sum();
    println!("sum: {}", sum); // 10

    let summed_vec: Vec<i32> = vec.iter().map(|x| x + 1).collect();
    println!("summed_vec: {:?}", summed_vec);  // [2, 3, 4, 5]
    println!("vec: {:?}", vec); // [1, 2, 3, 4]

    let filtered_vec: Vec<&i32> = vec.iter().filter(|x| x > &&3).collect();
    println!("filtered_vec: {:?}", filtered_vec); // [4]
    println!("vec: {:?}", vec); // [1, 2, 3, 4]
}