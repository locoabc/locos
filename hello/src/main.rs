mod classes;

fn main() {
    let rectangle = classes::Rectangle { width: 10.0, height: 5.0 };
    let circle = classes::Circle { radius: 3.0 };
    let triangle = classes::Triangle { base: 8.0, height: 4.0 };
    println!("Rectangle: width = {}, height = {}, area = {}", rectangle.width, rectangle.height, rectangle.area());
    println!("Circle: radius = {}, circumference = {}", circle.radius, circle.circumference());
    println!("Triangle: base = {}, height = {}, area = {}", triangle.base, triangle.height, triangle.area());
}
