/*
**********************************************************************
 * 
 * Author: Matthew Byram
 * 
 * Description: Takes in a csv file and parse through to create
 * teams. The output to terminal will be all the teams  
 * 
 * 
 **********************************************************************
 */

// Bringing in libraries
use rand::Rng;
use serde::Deserialize;

use config::{
    Config, 
    ConfigError, 
    File as ConfigFile, 
};

use std::{
    error::Error,
    fs::File as FsFile,
    process,
    collections::{HashSet, HashMap}
};

#[derive(Debug, Deserialize)]
struct UnitConfig {
    num_of_units: i32,
    unit_size: i32,
    csv_path: String,
}

// This run function will be used to looped through the csv and assign the characters
fn run(unit_config:UnitConfig) -> Result<(), Box<dyn Error>> {
    let mut index = 1;
    let mut character_map = HashMap::new();
    let mut seen_numbers:HashSet<i32> = HashSet::new();
    
    // Open csv file
    let file = FsFile::open(unit_config.csv_path)?;

    // Read the csv file
    let mut rdr = csv::Reader::from_reader(file);

    //Loops through the our reader holding csv and insert our characters from the csv into the Hashmap with desginated number
    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        let character = &record[0];
        character_map.insert(i, character.to_string());
    }

    // We are looping through the amount of units we are spitting out
    while index <= unit_config.num_of_units{
        let mut unit_num = 0;
        println!("Unit {:?}:", index);

        // Loop through the size of the units
        while unit_num < unit_config.unit_size{
            unit_num +=1;

            // Get a random number from the Hashmap
            let mut number = rand::thread_rng().gen_range(0..= character_map.len() - 1);
            let mut number_int:i32 = number as i32;

            // If random number already in Hashset, retry
            while seen_numbers.contains(&number_int){
                number = rand::thread_rng().gen_range(0..=character_map.len()-1);
                number_int = number as i32;
            }  
            
            // Add the number to Hashset
            seen_numbers.insert(number_int);

            // Print team to terminal
            println!("Member {:?}: {}", unit_num, character_map.get(&number).unwrap());  
        };
        
        index+=1;
    }

    Ok(())
}

fn load_config_file(file_name:&str)->Result<UnitConfig, ConfigError>{
        let builder = Config::builder()
        .add_source(ConfigFile::with_name(file_name))
        .build()?;
        
        builder.get::<UnitConfig>("units")
    }

// The main function
fn main() {
    let unit_config  = load_config_file("config.toml").unwrap();

    //Runs the run fuction with the amount of units we need 
    if let Err(err) = run(unit_config) {
        println!("{}", err);
        process::exit(1);
    }
}