fn main() {
    // Define a struct to represent a rectangle
    struct Rectangle {
        width: isize,
        height: isize,
    }

    // Create instances of Rectangle with specific dimensions
    let rect1 = Rectangle { width: 4, height: 6 };
    let rect2 = Rectangle { width: 8, height: 9 };

    // Calculate the area and perimeter of each rectangle
    println!("Area of first rectangle: {}", rect1.width * rect1.height);
    println!("Perimeter of first rectangle: {}", (rect1.width + rect1.height) * 2);

    println!("Area of second rectangle: {}", rect2.width * rect2.height);
    println!("Perimeter of second rectangle: {}", (rect2.width + rect2.height) * 2);
}
