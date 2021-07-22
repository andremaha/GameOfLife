extern crate rand;
use std::{thread, time};

fn main() {
    let mut world = [[0u8; 75]; 75];
    let mut generations = 0;

    
    for i in 0..74 {
        for j in 0..74 {
            if rand::random() {
                world[i][j] = 1;
            } else {
                world[i][j] = 0;
            }
        }
    }

    //println!("{:?}", world);
    println!("Living cells in the OLD world: {:?}", census(world));

    let newworld = generation(world);
    //println!("{:?}", newworld);
    println!("Living cells in the NEW world: {:?}", census(newworld));
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