#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;
    ((*x2 - *x1) * (*y2 - *y1)).abs()
}

fn square(p: &Point) -> Rectangle {
    Rectangle{
        top_left: Point{x: p.x, y: p.y},
        bottom_right: Point{x: p.x * 2.0, y: p.y * 2.0},
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);
    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);
    let mut bottom_right = Point { x: 5.2, ..point };
    bottom_right.y = 4f32;
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    println!(
        "rect: ({}, {}) ({}, {})",
        left_edge, top_edge, _rectangle.bottom_right.x, _rectangle.bottom_right.y
    );
    let sq = square(&Point{x: 4.2, y: 4.2});
    println!(
        "rect-square: ({}, {}) ({}, {})",
        sq.top_left.x, sq.top_left.y, sq.bottom_right.x, sq.bottom_right.y
    );
    println!("area: {}", rect_area(&_rectangle));
    let _unit = Unit;
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}
