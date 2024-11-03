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
use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    process,
    collections::{HashSet, HashMap}
};

// This run function will be used to looped through the csv and assign the characters
fn run(unit_size:i32) -> Result<(), Box<dyn Error>> {
    let mut index = 1;
    let mut character_map = HashMap::new();
    let mut seen_numbers:HashSet<i32> = HashSet::new();
    
    // Takes in the argument for the csv file name
    let file_path = get_first_arg()?;

    // Open csv file
    let file = File::open(file_path)?;

    // Read the csv file
    let mut rdr = csv::Reader::from_reader(file);

    //Loops through the our reader holding csv and insert our characters from the csv into the Hashmap with desginated number
    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        let character = &record[0];
        character_map.insert(i, character.to_string());
    }

    // We are looping through the amount of units we are spitting out
    while index <= unit_size{
        let mut unit_num = 0;
        println!("Unit {:?}:", index);

        // Custom code, but only 1 group has 5 units. Eventually all groups should have 5. 
        // Might be I should find a way to bring in unit sizes and restrictions where necessary for making this more generic
        let size = if index <=2 {
            5         
        }else{
            4
        };

        // Loop through the size of the units
        while unit_num < size{
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

// Checking the arguement being brought in, this case only 1 
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

// The main function
fn main() {

    //Runs the run fuction with the amount of units we need 
    if let Err(err) = run(10) {
        println!("{}", err);
        process::exit(1);
    }
}