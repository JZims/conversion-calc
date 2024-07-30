use clap::{command, Arg, Parser, Command};



// Without using built-in Parser
fn main() {
    let conversion = command!()
        .subcommand(
            Command::new("gravity-mod")
            .arg(
                Arg::new("gravity_modifer")
                .short('g')
                .long("gravity-mod")
                .alias("grav")
                .required(true)
                .value_parser(clap::value_parser!(f16))
                .help("Provide the gravity modifier (Specifig Gravity or Density) as a decimal here.")
            )
        )
        .subcommand(
            Command::new("conversion-type")
            .arg(
            Arg::new("conversion_type")
                .short('t')
                .long("type")
                .alias("type")
                .required(true)
                .help("What is the incomming value to be converted measured in? ('weight' or 'volume')")
                .value_parser(["weight", "volume", "Weight", "Volume", "g", "ml"])

             )
        )
        .subcommand(
            Command::new("unit-quantity")
            .arg(
            Arg::new("unit_quantity")
                .short('u')
                .long("units")
                .alias("units")
                .required(true)
                .help("How many units are being converted into the opposite measurement?")

             )
        );

}

// Using Parser

// #[derive(Parser)]
// struct Cli {
//     /// Either Volume or Weight.
//     conversion_input: String, 
//     /// Modifier used for material.
//     gravity_mod: f32
// }

// fn main () {
//     let args = Cli::parse();

//     if args.conversion_input.to_lowercase() == "volume" {
//         println!("You chose Volume! Your incoming value should be in mL and will be converted to g.")
//     }
//     if args.conversion_input.to_lowercase() == "weight" {
//         println!("You chose weight! Your incoming value should be in g and will be converted to mL.")
//     }

// }



