mod classes;
mod functions;
mod hello;

fn main() {
   // let name = String::from("Rusty");
   // greeting(name);

    hello::hello();
    let result = functions::add(5, 3);
    println!("Sum: {}", result);

    let result = functions::multiply(5, 3);
    println!("Product: {}", result);

    let rectangle = classes::Rectangle { width: 10.0, height: 5.0 };
    let circle = classes::Circle { radius: 3.0 };
    let triangle = classes::Triangle { base: 8.0, height: 4.0 };
    println!("Rectangle: width = {}, height = {}, area = {}", rectangle.width, rectangle.height, rectangle.area());
    println!("Circle: radius = {}, circumference = {}", circle.radius, circle.circumference());
    println!("Triangle: base = {}, height = {}, area = {}", triangle.base, triangle.height, triangle.area());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_test() {
        let want = String::from("Hello, World!");
        let result = hello_world();
        assert_eq!(want, result);
    }
    // ANCHOR_END: test

    // ANCHOR: greeting_test
    #[test]
    fn greeting_test() {
        let want = String::from("Hello, Rusty!");
        let name = String::from("Rusty");
        let result = greeting(name);
        assert_eq!(want, result);
    }
    // ANCHOR_END: greeting_test
}
