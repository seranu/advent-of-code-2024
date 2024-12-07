use std::env;
use std::process;
use std::error::Error;
use std::fs::read_to_string;

fn read_into_mat(file_name: &str) -> Vec<Vec<char>> {
   read_to_string(file_name) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|s| String::from(s).chars().collect())  // make each slice into a string
        .collect()  // gather them together into a vector 
}

fn count_xmas(file_name: &str) -> Result<u32, Box<dyn Error>> {
    let mat: Vec<Vec<char>>  = read_into_mat(&file_name);
    let mut count = 0;
    for (i, row) in mat.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if *el == 'X' {
                // horizontal
                if j + 3 < row.len() {
                    if row[j+1] == 'M' && row[j+2] == 'A' && row[j+3] == 'S' {
                        count += 1;
                    }
                }

                if j > 2 {
                    if row[j-1] == 'M' && row[j-2] == 'A' && row[j-3] == 'S' {
                        count += 1;

                    }
                }

                // vertical
                if i > 2 {
                    if mat[i-1][j] == 'M' && mat[i-2][j] == 'A' && mat[i-3][j] == 'S' {
                        count += 1;

                    }
                }

                if i + 3 < mat.len() {
                    if mat[i+1][j] == 'M' && mat[i+2][j] == 'A' && mat[i+3][j] == 'S' {
                        count += 1;

                    }
                }


                // diagonals
                if i > 2 && j > 2 {
                    if mat[i-1][j-1] == 'M' && mat[i-2][j-2] == 'A' && mat[i-3][j-3] == 'S' {
                       count += 1;
                    }
                }

                if i > 2 && j + 3 < row.len() {
                    if mat[i-1][j+1] == 'M' && mat[i-2][j+2] == 'A' && mat[i-3][j+3] == 'S' {
                        count += 1;
                    }
                }

                if i + 3 < mat.len() && j + 3 < row.len() {
                    if mat[i+1][j+1] == 'M' && mat[i+2][j+2] == 'A' && mat[i+3][j+3] == 'S' {
                        count += 1;
                    }
                }

                if i + 3 < mat.len() && j > 2 {
                    if mat[i+1][j-1] == 'M' && mat[i+2][j-2] == 'A' && mat[i+3][j-3] == 'S' {
                        count += 1;
                    }
                }
            }
        } 
    }

    return Ok(count);
}


fn count_xmax(file_name: &str) -> Result<u32, Box<dyn Error>> {
    let mat: Vec<Vec<char>>  = read_into_mat(&file_name);
    let mut count = 0;
    for (i, row) in mat.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if i == 0 || i == mat.len()-1 || j == 0 || j == mat.len()-1 {
                continue;
            }
            
            if *el == 'A' {
                if ((mat[i-1][j-1] == 'M' && mat[i+1][j+1] == 'S') ||
                    (mat[i-1][j-1] == 'S' && mat[i+1][j+1] == 'M')) &&
                    ((mat[i-1][j+1] == 'M' && mat[i+1][j-1] == 'S') ||
                     (mat[i-1][j+1] == 'S' && mat[i+1][j-1] == 'M')) {
                        count += 1;
                }
            }
        }
    }
    return Ok(count);
}

fn main() {
    let args: Vec<String>  = env::args().collect();

    if args.len() != 2 {
        println!("Invalid arguments");
        process::exit(1);
    }

    let res = count_xmas(&args[1]);
    println!("XMAS count = {:?}", res.unwrap());
    let ret = count_xmax(&args[1]);
    println!("XMAX count = {:?}", ret.unwrap());
}
