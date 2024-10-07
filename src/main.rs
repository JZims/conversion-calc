mod density_to_sg;

use clap::{command, Arg};
use crate::density_to_sg::density_to_specific_gravity;


// Without using built-in Parser
fn main() {
    let conversion = command!()
            .arg(
                Arg::new("gravity_modifier")
                .short('g')
                .long("gravity-mod")
                .alias("grav")
                .value_parser(clap::value_parser!(f32))
                .help("Provide the gravity modifier (Specific Gravity or Density) as a decimal here.")
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

             ).about("A lightweight CLI conversion tool for quickly converting volume and weight units using density and specific gravity measurements.")
            .get_matches();

    let modifier = conversion.get_one::<f32>("gravity_modifier").unwrap();
    let c_type = conversion.get_one::<String>("conversion_type").unwrap();
    let quantity = conversion.get_one::<f32>("unit_quantity").unwrap();


    if c_type == "weight" {

        let value:f32 = quantity / modifier;

        println!("The converted weight of {}g to volume(ml) is: {}ml", quantity, value )

    } else if c_type == "volume" {

         let value:f32 = quantity * modifier;
         let sg:f32 = density_to_specific_gravity(value);

        println!("The converted volume of {}ml to weight(g) is: {}g", quantity, value );
        println!("The specific gravity of the converted volume is: {}", sg);

    }


    
   

}

// Create a followup question to ask the user if they would like to convert Density to Specific Gravity or vice versa


