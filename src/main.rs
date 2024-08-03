// POINT BEHIND THE PROGRAM
// the idea is that you select the area, the program reads (then outputs)
// some sort of file with the possible encounters, then you pick the biome
// and the program shits out the encounter !! easy enough

use std::io;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use rand::Rng;
use serde::{ Serialize, Deserialize };

fn main() {
    // firstly: select the file
    let selected_area = select_area();
    // then read the selected file and select a biome from which to get the encounter
    let encounters = read_file(selected_area);
    // and then select the encounter from the given array
    let chosen_encounter = choose_encounter(encounters);
    // and shit out the encounter to the console! easy enough (?)
    println!("Your encounter is {chosen_encounter}");

    // println!("{}", std::env::current_dir().unwrap().display());
}

fn select_area() -> String {
    let mut province: String = "".to_string();
    let mut area: String = "".to_string();
    let mut filename: String = "./data/".to_string();
    let mut areas_amount: u8 = 0;
    let mut is_input_correct;

    println!("{}", std::env::current_dir().unwrap().display());

    loop {
        is_input_correct = true;

        println!("Please select the province you want your encounter for!");
        println!("  1. South Province");
        println!("  2. East Province");
        println!("  3. West Province");
        println!("  4. North Province");
        println!("  5. Great Crater");
        println!("  6. Others");

        io::stdin()
            .read_line(&mut province)
            .expect("Failed to read line");

        province = province.trim().to_string();

        if !province.parse::<u8>().is_ok() {
                println!("You have to input a valid number!");
                is_input_correct = false;
        } else {
            match province.parse().expect("Not a valid input") {
                1 => {
                    filename.push_str("s");
                    areas_amount = 6; // damn
                }
                2 => {
                    filename.push_str("e");
                    areas_amount = 3; 
                }
                3 => {
                    filename.push_str("w");
                    areas_amount = 3; 
                }
                4 => {
                    filename.push_str("n");
                    areas_amount = 3;
                }
                5 => {
                    filename.push_str("c");
                    areas_amount = 2;
                }
                6 => {
                    // each of the "others" will have a bespoke filename
                    // there's therefore no need for a prefix
                }
                _ => {
                    println!("Please input one of the given numbers!");
                    is_input_correct = false;
                }
            }
        }

        province = "".to_string();

        if is_input_correct {
            break;
        }
    }

    println!("Province has {areas_amount} areas");

    loop { // area selection!

        is_input_correct = true;

        if areas_amount == 0 { // is the area in the Others category?
            println!("Your area is in the Others category!");
            println!("Please select your area from the following table");
            println!("  1. Poco Path");
            println!("  2. Inlet Grotto");
            println!("  3. Alfornada Cavern");
            println!("  4. Tagtree Thicket");
            println!("  5. Asado Desert");
            println!("  6. Casseroya Lake");
            println!("  7. Socarrat Trail");
            println!("  8. Glaseado Mountain");
            println!("  9. Dalizapa Passage");

            io::stdin()
                .read_line(&mut area)
                .expect("Failed to read line");

            area = area.trim().to_string();

            if !area.parse::<u8>().is_ok() {
                println!("You have to input a valid number!");
                is_input_correct = false;
            } else { 
                match area.parse().expect("Not a valid input") {
                    1 => filename.push_str("pocopath"),
                    2 => filename.push_str("inletgrotto"),
                    3 => filename.push_str("alfornada"),
                    4 => filename.push_str("thicket"),
                    5 => filename.push_str("asadodesert"),
                    6 => filename.push_str("casseroya"),
                    7 => filename.push_str("socarrat"),
                    8 => filename.push_str("glaseado"),
                    9 => filename.push_str("dalizapa"),
                    _ => {
                        println!("Please input a valid area!");
                        is_input_correct = false;
                    }
                }
            }

            area.clear(); // resets because of loop situations
        } else {
            println!("Please select the area you want your encounter for!");
            println!("Inputting '0' will give you the corresponding sea encounter (and not Area Zero, fitting as it would be)");

            io::stdin()
                .read_line(&mut area)
                .expect("Failed to read line");

            area = area.trim().to_string();

            if !area.parse::<u8>().is_ok() {
                println!("You have to input a valid number!");
                is_input_correct = false;
            } else if area.parse::<u8>().expect("Not a valid input") > areas_amount {
                println!("Province has {areas_amount} areas; please input a number within");
                is_input_correct = false;
            } else if area.parse::<u8>().expect("Not a valid input") <= areas_amount {
                let area_u8: u8 = area.parse::<u8>().expect("Not a valid input");

                match area.parse::<u8>().expect("Not a valid input") {
                    0 => {
                        if filename == "c" {
                            println!("Area Zero has no possible sea encounter!");
                            is_input_correct = false;
                        } else {
                            filename.push_str("s");
                        }
                    },
                    _i if (1 ..= areas_amount).contains(&area_u8) => filename.push_str(&area), // TODO: please explain
                    _ => unreachable!(),
                }
            } else {
                println!("Unexpected item in bagging area! {}", area.parse::<u8>().expect("Not a valid input") );  
                is_input_correct = false;
            }
        }

        area = "".to_string();

        if is_input_correct {
            break;
        }
    }

    filename.push_str(".json");

    filename // is returned
}

fn read_file(filename: String) -> Encounters { 
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let encounters: Encounters = serde_json::from_reader(reader).expect("Error reading file");

    // we're also displaying the encounters here
    for (key, value) in &encounters.biomes {
        if !value.is_empty() {
            println!("  -- {}", key);
            for i in value {
                println!("{}", i);
            }
        }
    }

    encounters
}

fn choose_encounter(encounters: Encounters) -> String { // which should return a string
    let mut chosen_biome: String = "".to_string();

    println!("Please enter the biome you want your encounter for!");
    io::stdin()
        .read_line(&mut chosen_biome)
        .expect("Failed to read line");

    chosen_biome = chosen_biome.trim().to_string();

    let encounter_list = encounters.biomes.get(&chosen_biome).expect("Not found!");
    let chosen_encounter = &encounter_list[rand::thread_rng().gen_range(0..encounter_list.len())];

    chosen_encounter.to_string()
}

// structs

#[derive(Serialize, Deserialize, Debug)]
pub struct Encounters {
    biomes: HashMap<String, Vec<String>>,
}
