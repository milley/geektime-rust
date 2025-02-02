const MY_PI: f64 = std::f64::consts::PI;

struct Rectangle {
    a: f64,
    b: f64,
}

struct Circle {
    r: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

#[allow(dead_code)]
enum Shape {
    Rec(Rectangle),
    Cir(Circle),
    Tri(Triangle),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rec(r) => r.area(),
            Shape::Cir(c) => c.area(),
            Shape::Tri(t) => t.area(),
        }
    }
    fn circumference(&self) -> f64 {
        match self {
            Shape::Rec(r) => r.circumference(),
            Shape::Cir(c) => c.circumference(),
            Shape::Tri(t) => t.circumference(),
        }
    }
}

trait ShapeCalc {
    fn area(&self) -> f64;
    fn circumference(&self) -> f64;
}

impl ShapeCalc for Rectangle {
    fn area(&self) -> f64 {
        self.a * self.b
    }
    fn circumference(&self) -> f64 {
        self.a * 2.0 + self.b * 2.0
    }
}

impl ShapeCalc for Circle {
    fn area(&self) -> f64 {
        self.r * self.r * MY_PI
    }
    fn circumference(&self) -> f64 {
        2.0 * self.r * MY_PI
    }
}

impl ShapeCalc for Triangle {
    fn area(&self) -> f64 {
        let (a, b, c) = (self.a, self.b, self.c);
        let p = (a + b + c) / 2.0;
        (p * (p - a) * (p - b) * (p - c)).sqrt()
    }
    fn circumference(&self) -> f64 {
        self.a + self.b + self.c
    }
}

fn main() {
    let rec = Rectangle { a: 10.0, b: 5.0 };
    let shape = Shape::Rec(rec);
    println!("area is: {}, circumference is: {}", shape.area(), shape.circumference());

    let cir = Circle { r: 5.0 };
    let shape = Shape::Cir(cir);
    println!("area is: {}, circumference is: {}", shape.area(), shape.circumference());

    let tri = Triangle { a: 10.0, b: 5.0, c: 5.0 };
    let shape = Shape::Tri(tri);
    println!("area is: {}, circumference is: {}", shape.area(), shape.circumference());
}
