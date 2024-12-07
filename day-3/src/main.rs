use std::fs;
use std::env;
use std::process;
use std::error::Error;

static MUL: &str = "mul(";
static DO: &str = "do()";
static DONT: &str = "don't()";

fn parse_int(input: &Vec<char>, i: &mut usize) -> Result<u32, Box<dyn Error>> {
    let mut char_count = 0;
    let mut nr = 0;
    let radix = 10;
    while char_count < 3 && i < &mut input.len() {
        if !input[*i].is_digit(radix) {
            break;
        }
        let digit = input[*i].to_digit(radix).unwrap();

        if char_count == 0 && digit == 0 {
            break;
        }

        nr = nr * 10 + digit; 
        *i += 1;
        char_count += 1;
    }

    if char_count == 0 {
        return Err("Failed to parse int".into());
    }

    return Ok(nr);
}

fn parse_multiply(input: &Vec<char>, idx: usize) -> Result<u32, Box<dyn Error>> {

    let mut i: usize = idx;
    i += MUL.len();

    let lhs = parse_int(&input, &mut i)?;

    if input[i] != ',' {
        return Err("failed to parse multiplication".into());
    }

    i += 1;

    let rhs = parse_int(&input, &mut i)?;

    if input[i] != ')' {
        return Err("failed to parse multiplication".into());
    }

    return Ok(lhs*rhs);
}

fn get_interesting_indices(input: &String) -> Vec<(usize, &str)> {
    let mut indices: Vec<_> = input.match_indices(MUL).collect();
    let mut dos: Vec<_> = input.match_indices(DO).collect();
    let mut donts: Vec<_> = input.match_indices(DONT).collect();
    indices.append(&mut dos);
    indices.append(&mut donts);
    indices.sort();
    return indices;
}

fn mul_result(file_name: &str) -> Result<u32, Box<dyn Error>> {
    let input: String = fs::read_to_string(file_name)?;
    let iv: Vec<char> = input.chars().collect();
    let mut res: u32 = 0;
    let mut enabled: bool = true;
    println!("Input: {:?}", input);
    let occs: Vec<_> = get_interesting_indices(&input);
    for occ in occs {
        if input[occ.0..].starts_with(DONT) {
            enabled = false;
            continue;
        }

        if input[occ.0..].starts_with(DO) {
            enabled = true;
            continue;
        }

        if enabled {
            match parse_multiply(&iv, occ.0) {
                Err(e) => println!("{e}"),
                Ok(mul_res) => res += mul_res,
            }
        }
    }

    return Ok(res);
}

fn main() {
    let args: Vec<String>  = env::args().collect();

    if args.len() != 2 {
        println!("Invalid arguments");
        process::exit(1);
    }

    let res = mul_result(&args[1]);
    println!("Multiplication result = {:?}", res.unwrap());
}
