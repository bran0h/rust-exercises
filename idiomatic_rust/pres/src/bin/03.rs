trait Render {
    fn paint(&self);
}

struct Circle;
impl Render for Circle {
    fn paint(&self) {
        println!("Circle");
    }
}

struct Rectangle;
impl Render for Rectangle {
    fn paint(&self) {
        println!("Rectangle");
    }
}

fn main() {
    let mut shapes: Vec<Box<dyn Render>> = Vec::new();
    let circle = Box::new(Circle);
    shapes.push(circle);
    let rect = Box::new(Rectangle);
    shapes.push(rect);
    shapes.iter().for_each(|shape| shape.paint());
}
