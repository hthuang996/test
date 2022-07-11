trait Area {
    fn calc_area(&self) -> u16;
}

struct Triangle {
    bottom: u8,
    height: u8,
}

impl Area for Triangle {
    fn calc_area(&self) -> u16 {
        (self.bottom as u16) * (self.height as u16) / 2
    }
}

struct Rectangle {
    length: u8,
    width: u8,
}

impl Area for Rectangle {
    fn calc_area(&self) -> u16 {
        (self.length as u16 ) * (self.width as u16)
    }
}

struct Square {
    length: u8,
}

impl Area for Square {
    fn calc_area(&self) -> u16 {
        (self.length as u16) * (self.length as u16)
    }
}

fn calc_area<T: Area>(shape: T) -> u16 {
    shape.calc_area()
}

fn main() {
    let t = Triangle{
        bottom: 5,
        height: 6,
    };
    println!("Area of the triangle: {}", calc_area(t));
    let r = Rectangle{
        length: 5,
        width: 6,
    };
    println!("Area of the rectangle: {}", calc_area(r));
    let s = Square{
        length: 50,
    };
    println!("Area of the square: {}", calc_area(s));
}
