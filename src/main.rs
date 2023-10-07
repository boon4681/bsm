use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

type INSTRUCTION<'a> = (String, Vec<&'a str>);
type FUNCTION<'a> = (String, Vec<INSTRUCTION<'a>>);

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

fn check_function<'a>(funcs: &Vec<FUNCTION>, input: &str) -> Result<(), &'a str> {
    for (k, _) in funcs {
        if k.eq(input) {
            return Ok(());
        }
    }
    panic!("Function {} is not defined", input)
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

fn find_function<'a>(funcs: &Vec<FUNCTION>, name: &str) -> Result<usize, &'a str> {
    let mut i = 0;
    for (k, _) in funcs {
        if k.eq(name) {
            return Ok(i);
        }
        i += 1;
    }
    panic!("Function {} not found", name)
}

fn parse(input: &str) -> Result<Vec<FUNCTION>, &str> {
    let mut funcs: Vec<FUNCTION> = Vec::new();
    let mut insts: Vec<INSTRUCTION> = Vec::new();
    let mut name = "START";
    let mut empty = 0;
    for line in input.split("\n") {
        let trimmed = line.trim();
        if trimmed.len() > 1 && trimmed.ends_with(":") {
            for (k, v) in &funcs {
                if k.eq(name) {
                    return Err("Your function fuck");
                }
            }
            if insts.len() > empty {
                funcs.push((name.to_string(), insts.clone()));
                insts.clear();
            } else {
                insts.clear();
            }
            name = &trimmed[0..trimmed.len() - 1].trim();
            empty = 0;
            continue;
        }
        if trimmed.len() < 4 {
            insts.push(("EMPTY".to_string(), Vec::new()));
            empty += 1;
            continue;
        }
        let key = trimmed[0..4].trim();
        let params: Vec<&str> = trimmed[key.len() + 1..].split(",").map(|v| v.trim()).collect();
        match key {
            "SET" | "ADD" | "SUB" | "MULT" | "DIV" | "CMP" => {
                check_params(2, params.len())?;
                check_register(params[0])?;
                check_value(params[1])?;
                insts.push((key.to_string(), params))
            }
            "JMPZ" | "JMP" => {
                check_params(1, params.len())?;
                check_number(params[0])?;
                insts.push((key.to_string(), params))
            }
            "PRNT" => {
                check_params(1, params.len())?;
                check_value(params[0])?;
                insts.push((key.to_string(), params))
            }
            "CALL" => {
                check_params(1, params.len())?;
                check_function(&funcs, params[0])?;
                insts.push((key.to_string(), params))
            }
            _ => {
                return Err("Code fuck");
            }
        }
    }
    funcs.push((name.to_string(), insts.clone()));
    insts.clear();
    Ok(funcs)
}

fn bsm(input: &str) -> Result<(), &str> {
    let mut A: i32 = 0;
    let mut B: i32 = 0;
    let mut C: i32 = 0;
    let mut T: i32 = 0;

    let funcs = parse(input)?;
    let start_fn_idx = find_function(&funcs, "START")?;
    let mut stack: Vec<(usize, usize)> = vec![(start_fn_idx, 0)];
    // runtime
    while stack.len() > 0 {
        let (fn_idx, mut index) = match stack.pop() {
            Some(e) => e,
            None => {
                break;
            }
        };
        while index < funcs[fn_idx].1.len() {
            let (inst, params) = &funcs[fn_idx].1[index];
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
                        index = to_number(params[0])? as usize - 1;
                    }
                }
                "JMPZ" => {
                    if T == 0 {
                        index = to_number(params[0])? as usize - 1;
                    }
                }
                "PRNT" => {
                    println!("{}", convert(A, B, C, params[0])?);
                }
                "CALL" => {
                    let idx = find_function(&funcs, params[0])?;
                    stack.push((fn_idx, index));
                    stack.push((idx, 0));
                    break;
                }
                "EMPTY" => {}
                _ => {}
            }
        }
    }
    Ok(())
}
