use clap::Parser;


#[derive(Parser)]
struct Cli {
    /// Either Volume or Weight.
    conversion_input: String, 
    /// Modifier used for material.
    gravity_mod: f32
}

fn main () {
    let args = Cli::parse();

    if args.conversion_input.to_lowercase() == "volume" {
        println!("You chose Volume! Your incoming value should be in mL and will be converted to g.")
    }
    if args.conversion_input.to_lowercase() == "weight" {
        println!("You chose weight! Your incoming value should be in g and will be converted to mL.")
    }

}



