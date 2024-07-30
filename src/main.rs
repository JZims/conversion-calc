use clap::{command, Arg, Parser, Command};



// Without using built-in Parser
fn main() {
    let conversion = command!()
            .arg(
                Arg::new("gravity_modifier")
                .short('g')
                .long("gravity-mod")
                .alias("grav")
                .value_parser(clap::value_parser!(f32))
                .help("Provide the gravity modifier (Specifig Gravity or Density) as a decimal here.")
                .global(true)
            )
            .arg(
            Arg::new("conversion_type")
                .short('t')
                .long("type")
                .alias("type")
                .help("What is the incomming value to be converted measured in? ('weight' or 'volume')")
                .value_parser(["weight", "volume", "Weight", "Volume", "g", "ml"])
                .global(true)

             )
       
            .arg(
            Arg::new("unit_quantity")
                .short('u')
                .long("units")
                .alias("units")
                .help("How many units are being converted into the opposite measurement?")
                .global(true)
                .value_parser(clap::value_parser!(f32))

             )
            .get_matches();

    let modifier = conversion.get_one::<f32>("gravity_modifier").unwrap();
    let c_type = conversion.get_one::<String>("conversion_type").unwrap();
    let quantity = conversion.get_one::<f32>("unit_quantity").unwrap();

    if c_type == "weight" {

        let value:f32 = quantity / modifier;

        println!("The converted weight of {}g to volume(ml) is: {}ml", quantity, value )

    } else if c_type == "volume" {

         let value:f32 = quantity * modifier;

        println!("The converted volume of {}ml to weight(g) is: {}g", quantity, value )
    }
    
   

}

// Using built-in Parser

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



