// ********* second refactor with structs *********

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area is {}", area(&rectangle1))
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// ********* first refactor *********
// fn main() {
//     let rectangle1 = (30, 50);

//     println!("area is {}", area(rectangle1))
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// ********* before refactoring with structs *********
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!("area is {}", area(width1, height1));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
