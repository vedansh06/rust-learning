//* Implementing Structs

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };
    print!("The area of the rectangle is {}", rect.area());
}



struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self, num: i32) -> i32 {
        2 * (self.width + self.height)
    }

    fn debug() -> i32 {
        return 1;
    }
}

fn main() {
    let rect1 = Rect {
        width: 10,
        height: 20,
    };

    println!("area is {}", rect1.area());
    println!("perimeter is {}", rect1.perimeter(1));
    println!("debug is {}", Rect::debug());
}
