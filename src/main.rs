use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    process,
    collections::{HashSet, HashMap}
};

use rand::Rng;

fn run(unit_size:i32) -> Result<(), Box<dyn Error>> {
    let mut index = 1;
    let mut character_map = HashMap::new();
    let mut seen_numbers:HashSet<i32> = HashSet::new();
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        let character = &record[0];
        character_map.insert(i, character.to_string());
    }

    while index <= unit_size{
        let mut unit_num = 0;
        println!("Unit {:?}:", index);

        let size = if index == 1{
            5         
        }else{
            4
        };

        while unit_num < size{
            unit_num +=1;
            let mut number = rand::thread_rng().gen_range(0..=character_map.len()-1);
            let mut number_int:i32 = number as i32;

            while seen_numbers.contains(&number_int){
                number = rand::thread_rng().gen_range(0..=character_map.len()-1);
                number_int = number as i32;
                
            }           
            seen_numbers.insert(number_int);
            println!("Member {:?}: {}", unit_num, character_map.get(&number).unwrap());
            
        };
        
        index+=1;
    }

    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run(10) {
        println!("{}", err);
        process::exit(1);
    }
}