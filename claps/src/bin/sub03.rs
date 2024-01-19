// extern crate clap;
use clap::{arg, command, Command};

fn main() {
    
    let matches = command!() // requires `cargo` feature
        .help_template("{before-help}{name} {version}
{author-with-newline}{about-with-newline}
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
}
