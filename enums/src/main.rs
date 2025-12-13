//? Enums

enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let my_direction = Direction::North;
    let new_direction = my_direction; 
    move_around(new_direction);
}

fn move_around(direction: Direction) {

}



Define an enum called Shape
enum Shape {
    Circle(f64),  
    Square(f64),  
    Rectangle(f64, f64),  
}


fn calculate_area(shape: Shape) -> f64 {
    
    return 0
}

fn main() {
  
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);
    
}



enum Shape {
    circle(f64),
    square(f64)
    rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
match shape{
    Shape::circle(radius) => std::f64::consts::PI * radius*radius
    Shape::square(side_length) => side_length * side_length
    Shape::rectangle(width, height) => width * height
}
}

fn main(){
    let circle = Shape::circle(5.0);
    let square= Shape::square(4.0);
    let rectangle= Shape::rectangle(3.0,6.0);
}



enum Direction{
    North,
    South,
    East,
    West
}

fn main(){
    ley my_direction = Direction::North;
    let new_direction = my_direction;
    move_around(new_direction)
}

fn move_around(direction: Direction){

}



enum Shape {
    Rectangle(f64, f64), 
    Circle(f64),         
}

fn main() {
    let rect = Shape::Rectangle(1.0, 2.0);
    calculate_area(rect);
    let circle = Shape::Circle(1.0);
    calculate_area(circle);
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}