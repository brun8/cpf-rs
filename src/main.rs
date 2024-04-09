use std::{collections::HashMap, fmt::format};

use clap::Parser;
use rand::prelude::*;

#[derive(Parser, Debug)]
#[command(name = "cpf.rs")]
#[command(version = "1.0")]
#[command(about = "gerador de cpf na linha de comando", long_about = None)]
struct Cli {
    /// número de CPFs
    #[arg(short = 'n', default_value_t = 1)]
    number: u32,
    /// estado de emissão do CPF
    #[arg(short = 'e', long = "estado", default_value_t = String::from(""))]
    state: String,
    /// mostrar CPF com pontuação
    #[arg(short = 'p', default_value_t = false)]
    punctuated: bool,
    /// completar CPF incompleto
    #[arg(short = 'c', long = "complete", default_value_t = String::from(""))]
    partial: String,
}

fn main() {
    let cli = Cli::parse();
    let mut region_map = HashMap::<String, i32>::new();
    populate_regions_map(&mut region_map);

    let region: i32 = match region_map.get(&cli.state.to_uppercase()) {
        Some(i) => *i,
        None => -1,
    };

    if cli.partial.len() > 0 {
        println!("{}", complete_cpf(cli.partial, region));
    } else {
        for _ in 0..cli.number {
            println!("{}", generate_cpf(cli.punctuated, region));
        }
    }
}

fn complete_cpf(partial: String, region: i32) -> String {
    if partial.len() >= 11 {
        return partial;
    }

    let mut partial_vec: Vec<u32> = partial.chars().map(|n| n.to_digit(10).unwrap()).collect();

    if partial_vec.len() < 9 {
        for _ in 0..(9 - partial_vec.len()) {
            let n: u32 = random::<u32>() % 9;
            partial_vec.push(n);
        }
    }

    if region != -1 {
        partial_vec[8] = match u32::try_from(region) {
            Ok(num) => num,
            _ => 1,
        };
    }

    if partial_vec.len() < 10 {
        let d1 = generate_digit(&partial_vec);
        partial_vec.push(d1);
    }

    if partial_vec.len() < 11 {
        let d2 = generate_digit(&partial_vec[1..].to_owned());
        partial_vec.push(d2);
    }

    let res: String = partial_vec.iter().map(|num| num.to_string()).collect();
    res
}

fn generate_cpf(punctuated: bool, region: i32) -> String {
    let mut nums: Vec<u32> = Vec::new();
    for _ in 0..9 {
        let n: u32 = random::<u32>() % 9;
        nums.push(n);
    }

    if region != -1 {
        nums[8] = match u32::try_from(region) {
            Ok(num) => num,
            _ => 1,
        };
    }

    let d1 = generate_digit(&nums);
    nums.push(d1);

    let d2 = generate_digit(&nums[1..].to_owned());
    nums.push(d2);

    let res: String = nums.iter().map(|num| num.to_string()).collect();

    if punctuated {
        return format(format_args!(
            "{}.{}.{}-{}",
            &res[0..3],
            &res[3..6],
            &res[6..9],
            &res[9..]
        ));
    }

    return res;
}

fn generate_digit(nums: &Vec<u32>) -> u32 {
    let mut mul = 10;
    let sum = nums.iter().fold(0, |acc, num| {
        mul -= 1;
        acc + num * (mul + 1)
    });
    if sum % 11 < 2 {
        0
    } else {
        11 - (sum % 11)
    }
}

fn populate_regions_map(hm: &mut HashMap<String, i32>) {
    let states_per_region = HashMap::<i32, String>::from([
        (0, "RS".into()),
        (1, "DF, GO, MS, MT, TO".into()),
        (2, "AC, AM, AP, PA, RO, RR".into()),
        (3, "CE, MA, PI".into()),
        (4, "AL, PB, PE, RN".into()),
        (5, "BA, SE".into()),
        (6, "MG".into()),
        (7, "ES, RJ".into()),
        (8, "SP".into()),
        (9, "PR, SC".into()),
    ]);

    states_per_region.iter().for_each(|(k, v)| {
        let states: Vec<&str> = v.split(", ").collect();
        states.iter().for_each(|state| {
            hm.insert(state.to_string(), *k);
        })
    });
}
