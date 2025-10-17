enum Shape {
    Circle(f64),          // radius
    Rectangle(f64, f64),  // width, height
    Square(f64),          // side length

    Triangle(f64, f64, f64), // sides a, b, c
    Polygon(Vec<(f64, f64)>), // list of vertices
}

fn main(){
    let my_shape = Shape::Circle(5.0);
}

fn print_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14159 * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
        Shape::Polygon(vertices) => {
            let n = vertices.len();
            let mut area = 0.0;
            for i in 0..n {
                let (x1, y1) = vertices[i];
                let (x2, y2) = vertices[(i + 1) % n];
                area += x1 * y2 - x2 * y1;
            }
            area.abs() / 2.0
        }
    }
}   