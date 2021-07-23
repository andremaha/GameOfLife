extern crate rand;
extern crate termion;
use std::{thread, time, env};
use std::fs::File;
use std::io::{BufRead, BufReader};
use termion::clear;
use std::io::prelude::*;

fn main() {
    let mut world = [[0u8; 75]; 75];
    let mut generations = 0;

    let args: Vec<String> = env::args().collect();
    
    // No file provided to populate the world - use random population
    if args.len() < 2 {
        for i in 0..74 {
            for j in 0..74 {
                if rand::random() {
                    world[i][j] = 1;
                } else {
                    world[i][j] = 0;
                }
            }
        }
    } else {
        let filename = env::args().nth(1).unwrap();
        world = populate_from_file(filename);
    }

    for _gens in 0..10 {
        let temp = generation(world);
        world = temp;
        generations += 1;
        println!("{}", clear::All);
        displayworld(world);
        println!("Population at generation {g} is {c}", g = generations, c = census(world));
        thread::sleep(time::Duration::from_secs(2));
    }

    // Save the last generation to file as field
    write_world_to_file(world, String::from("last_field.txt"));
    // Save the last generatio to file as coorinates
    write_world_coordinates_to_file(world, String::from("world.txt"));

}


/// # Writes the current world as the filed to a file
fn write_world_to_file(world: [[u8; 75]; 75], filename: String) {
    let mut file = File::create(filename).unwrap();
    for i in 0..74 {
        for j in 0..74 {
            if world[i][j] == 1 {
                write!(file, "{}", "*").unwrap();
            } else {
                write!(file, "{}", " ").unwrap();
            }
            
        }
        write!(file, "{}", "\n").unwrap();
    }

}

/// # Writes the current world into the coordinates file
/// 
/// The file can be used to regenerate the field 
/// By passing filename as an argument when running the program
fn write_world_coordinates_to_file(world: [[u8; 75]; 75], filename: String) {
    let mut file = File::create(filename).unwrap();
    for i in 0..74 {
        for j in 0..74 {
            if world[i][j] == 1 {
                write!(file, "{} {}\n", i, j).unwrap();
            }
        }
    }

}

/// # Creates a world from coordinates in the file
/// 
/// File format with coordinates of the cells in the field that have life in them
/// 0 5
/// 0 7
/// 0 18
fn populate_from_file(filename: String) -> [[u8; 75]; 75] {
    let mut newworld = [[0u8; 75]; 75];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let mut words = l.split_whitespace();
        let left = words.next().unwrap();
        let right = words.next().unwrap();
        pairs.push((left.parse::<usize>().unwrap(), right.parse::<usize>().unwrap()));
    }

    for i in 0..74 {
        for j in 0..74 {
            newworld[i][j] = 0;
        }
    }

    println!("{:?}", pairs);

    for (x, y) in pairs {
        newworld[x][y] = 1;
    }

    newworld
}

/// # Counts all living cells 
/// 
/// If there are no cells left in the world - 
/// there is no point in running the game. 
/// No life can be born in an empty world.
fn census(world: [[u8; 75]; 75]) -> u16 {
    let mut count = 0;

    for i in 0..74 {
        for j in 0..74 {
            if world[i][j] == 1 {
                count += 1;
            }
        }
    }

    count
}

/// # Moves the world to the new evolutionary generation 
/// 
/// Some cells die, some live, some resuurect. 
/// Following the rules of The Game. 
/// 
/// * If a cell is currently alive but it has fewer than two neighbors, it will die because of lack of support
/// * If a cell is currently alive and has two or three neighbors, it will survive to the next generation
/// * If a cell is currently alive and has more than three neighbors, it dies from overpopulation (lack of resources)
/// * If a cell currently dead but has exaclty three neighbors, it will come back to life
fn generation(world: [[u8; 75]; 75]) -> [[u8; 75]; 75] {
    let mut newworld = [[0u8; 75]; 75];

    for i in 0..74 {
        for j in 0..74 {
            
            // How to count cells around cell in question?
            // 
            // Where i is row
            // Where j is column
            // Where X is cell in question
            // 
            // Add all the cells around X with the following cooridnates
            // ----|----|----
            // i-1 |i-1 |i-1
            // j-1 |j   |j+1
            // --------------
            // i   |    |i
            // j-1 | X  |j+1
            // --------------
            // i+1 |i+1 |i+1
            // j-1 |j   |j+1
            // 
            // So the calculation for X when it is somewhere in the middle of the world would be just adding them all up
            // The problem is: overflows[0-1], when X is somewhere on the ranges
            // 
            // Following ranges are possible:
            // X is in the first row (no previous row calculations possible) 
            // X is on the last row (no next row calculations possible)
            // X is on the first column (no previos column calculations possible)
            // X is on the last column (no next column calculations possible)
            // Combination of the above - when X is in one of the corners
            
            let mut count = 0;

            if i > 0 && j > 0 {
                count = count + world[i-1][j-1];
            }
            if i > 0 {
                count = count + world[i-1][j];
            }
            if i > 0 && j < 74 {
                count = count + world[i-1][j+1];
            }
            if j > 0 {
                count = count + world[i][j-1];
            }
            if j < 74 {
                count = count + world[i][j+1];
            }
            if i < 74 && j > 0 {
                count = count + world[i+1][j-1];
            }   
            if i < 74 {
                count = count + world[i+1][j];
            }
            if i < 74 && j < 74 {
                count = count + world[i+1][j+1];
            }
                
            
            newworld[i][j] = 0;

            // If a cell is curretly alive but it has fewer than two neighbors, it will die because of lack of support
            if (world[i][j] == 1) && (count < 2)  {
                newworld[i][j] = 0;
            }
            // If a cell is currently alive but has two or three neighbors, it will survive to the next generation
            if world[i][j] == 1 && (count == 2 || count == 3) {
                newworld[i][j] = 1;
            }
            // If a cell is currently alive and has more than three neighbors, it dies from overpopulation (lack of resources)
            if (world[i][j] == 1) && (count > 3) {
                newworld[i][j] = 0;
            }
            // If a cell currently dead but has exaclty three neighbors, it will come back to life
            if (world[i][j] == 0) && (count == 3) {
                newworld[i][j] = 1;
            }
        }
    }

    newworld
}

/// # Display the current evolution of the world
/// 
fn displayworld(world: [[u8; 75]; 75]) {
    for i in 0..74 {
        for j in 0..74 {
            if world[i][j] == 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}