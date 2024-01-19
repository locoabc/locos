use clap::{arg, command, Command};

mod classes;
mod functions;
mod hello;


fn main() {
   // let name = String::from("Rusty");
   // greeting(name);


    let matches = command!() // requires `cargo` feature
        .help_template("{before-help}{name} {version} {author-with-newline} {about-with-newline}
{usage-heading} [Options] [Commands] [Options]
{all-args}{after-help}")
        .version("1.1")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("create")
                .about("C-RUD ...")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("retrieve")
                .about("C-R-UD ...")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("update")
                .about("CR-U-D ...")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("delete")
                .about("CRU-D ...")
                .arg(arg!([NAME])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => println!(
            "'myapp create' was used, name is: {:?}",
            sub_matches.get_one::<String>("NAME")
        ),
        Some(("retrieve", sub_matches)) => println!(
            "'myapp retrieve' was used, name is: {:?}",
            sub_matches.get_one::<String>("NAME")
        ),
        Some(("update", sub_matches)) => println!(
            "'myapp update' was used, name is: {:?}",
            sub_matches.get_one::<String>("NAME")
        ),
        Some(("delete", sub_matches)) => println!(
            "'myapp delete' was used, name is: {:?}",
            sub_matches.get_one::<String>("NAME")
        ),

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }


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
