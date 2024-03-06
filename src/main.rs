use std::fmt::format;

use clap::Parser;
use rand::prelude::*;

#[derive(Parser, Debug)]
#[command(name = "cpf.rs")]
#[command(version = "1.0")]
#[command(about = "gerador de cpf na linha de comando", long_about = None)]
struct Cli {
    #[arg(short = 'n', default_value_t = 1)]
    number: u32,
    // TODO: validar entre 0 e 9
    // TODO: mostrar opcoes na mensagem de ajuda
    #[arg(short = 'r', default_value_t = -1)]
    region: i32,
    #[arg(short = 'p', default_value_t = false)]
    punctuated: bool,
}

fn main() {
    let cli = Cli::parse();

    for _ in 0..cli.number {
        println!("{}", generate_cpf(cli.punctuated, cli.region));
    }
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
