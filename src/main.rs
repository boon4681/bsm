use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("./main.bsm").expect("");

    bsm(input.as_str()).expect("TODO: panic message");
}

fn check_register(input: &str) -> Result<(), &str> {
    match input {
        "A" | "B" | "C" => Ok(()),
        _ => Err("FUCK"),
    }
}

fn check_value(input: &str) -> Result<(), &str> {
    match check_register(input) {
        Ok(e) => {
            return Ok(());
        }
        Err(_) => {}
    }
    Ok(check_number(input)?)
}

fn check_number(input: &str) -> Result<(), &str> {
    match i32::from_str(input) {
        Ok(value) => Ok(()),
        Err(_) => Err("VALUE ERROR"),
    }
}

fn check_params<'a>(len: usize, size: usize) -> Result<(), &'a str> {
    if size != len {
        return Err("PARAMS ERROR");
    }
    Ok(())
}

fn convert(A: i32, B: i32, C: i32, value: &str) -> Result<i32, &str> {
    match value {
        "A" => Ok(A),
        "B" => Ok(B),
        "C" => Ok(C),
        _ => Ok(to_number(value)?),
    }
}

fn to_number(input: &str) -> Result<i32, &str> {
    match i32::from_str(input) {
        Ok(v) => Ok(v),
        Err(_) => Err("CONVERT ERROR"),
    }
}

fn parse(input: &str) -> Result<Vec<(String, Vec<&str>)>, &str> {
    let mut list: Vec<(String, Vec<&str>)> = Vec::new();
    for line in input.split("\n") {
        let trimmed = line.trim();
        if trimmed.len() < 4 {
            list.push(("EMPTY".to_string(), Vec::new()));
            continue;
        }
        let key = trimmed[0..4].trim();
        let params: Vec<&str> = trimmed[4..].split(",").map(|v| v.trim()).collect();
        match key {
            "SET" | "ADD" | "SUB" | "MULT" | "DIV" | "CMP" => {
                check_params(2, params.len())?;
                check_register(params[0])?;
                check_value(params[1])?;
                list.push((key.to_string(), params))
            }
            "JMPZ" | "JMP" => {
                check_params(1, params.len())?;
                check_number(params[0])?;
                list.push((key.to_string(), params))
            }
            "PRNT" => {
                check_params(1, params.len())?;
                check_value(params[0])?;
                list.push((key.to_string(), params))
            }
            _ => {
                return Err("Code fuck");
            }
        }
    }
    Ok(list)
}

fn bsm(input: &str) -> Result<(), &str> {
    let mut A: i32 = 0;
    let mut B: i32 = 0;
    let mut C: i32 = 0;
    let mut T: i32 = 0;

    let insts = parse(input)?;
    let len = insts.len();
    let mut index = 0;

    // runtime
    while index != len {
        let (inst, params) = &insts[index];
        index += 1;
        match inst.as_str() {
            "SET" => {
                match params[0] {
                    "A" => A = convert(A, B, C, params[1])?,
                    "B" => B = convert(A, B, C, params[1])?,
                    "C" => C = convert(A, B, C, params[1])?,
                    _ => {}
                }
            }
            "ADD" => {
                match params[0] {
                    "A" => A += convert(A, B, C, params[1])?,
                    "B" => B += convert(A, B, C, params[1])?,
                    "C" => C += convert(A, B, C, params[1])?,
                    _ => {}
                }
            }
            "SUB" => {
                match params[0] {
                    "A" => A -= convert(A, B, C, params[1])?,
                    "B" => B -= convert(A, B, C, params[1])?,
                    "C" => C -= convert(A, B, C, params[1])?,
                    _ => {}
                }
            }
            "MULT" => {
                match params[0] {
                    "A" => A *= convert(A, B, C, params[1])?,
                    "B" => B *= convert(A, B, C, params[1])?,
                    "C" => C *= convert(A, B, C, params[1])?,
                    _ => {}
                }
            }
            "DIV" => {
                match params[0] {
                    "A" => A /= convert(A, B, C, params[1])?,
                    "B" => B /= convert(A, B, C, params[1])?,
                    "C" => C /= convert(A, B, C, params[1])?,
                    _ => {}
                }
            }
            "CMP" => {
                T = convert(A, B, C, params[0])? - convert(A, B, C, params[1])?;
            }
            "JMP" => {
                if T != 0 {
                    index = to_number(params[0])? as usize;
                }
            }
            "JMPZ" => {
                if T == 0 {
                    index = to_number(params[0])? as usize;
                }
            }
            "PRNT" => {
                println!("{}", convert(A, B, C, params[0])?);
            }
            "EMPTY" => {}
            _ => {}
        }
    }
    // println!("{:?}", insts);
    // println!("{} {} {}", A, B, C);
    Ok(())
}
