use std::{io::{self, Stdin}, f32::MIN};

fn main() {
    
    let mut buffer: String = String::new();
    let stdin: Stdin = io::stdin();

    println!("give me smth to calculate!");
    match stdin.read_line(&mut buffer) {
        Ok(n) => {
            println!("Read {}-bytes, now sanitizing {}", n, buffer);
        }
        Err(why) => println!("Got an error {}", why),
    }

    let parts: Vec<&str> = buffer.split(" ").collect();
    if parts.len() < 3 || parts.len() % 2 == 0 {
        println!("Form not supproted! Please give your input like\noperand [+|-|*|/|**] other_operand\nyes, with the spaces");
        return ;
    }
    
    let mut sum: f32 = get_number_from_string(parts[0]);
    
    let mut i: usize = 1;
    while i < parts.len() - 1 {
        sum = get_temp_result(sum, get_number_from_string(parts[i + 1].trim()), parts[i]);
        i += 2;
    }

    println!("Total result of {:#?} = {}", parts, sum);
}

fn get_temp_result(temp_sum: f32, next_num: f32, current_op: &str) -> f32 {

    // idk how one could have handled this better
    match current_op {

        "+"   => add_nums(temp_sum, next_num),
        "-"   => sub_nums(temp_sum, next_num),
        "*"   => mul_nums(temp_sum, next_num),
        "/"   => div_nums(temp_sum, next_num),
        "**"  => pow_nums(temp_sum, next_num),
        "%"   => mod_nums(temp_sum, next_num),
        _ => MIN    
    }
}

fn get_number_from_string(string: &str) -> f32 {

    let mut default_num: f32 = MIN;
    match string.parse::<f32>() {
        Ok(parsed_num) => default_num = parsed_num,
        Err(why) => println!("{} could not be parsed into f32, bc {}", string, why),
    };

    return default_num;
}

fn add_nums(first_num: f32, second_num: f32) -> f32 {

    first_num + second_num
}

fn sub_nums(first_num: f32, second_num: f32) -> f32 {

    first_num - second_num
}

fn mul_nums(first_num: f32, second_num: f32) -> f32 {

    first_num * second_num
}

fn div_nums(first_num: f32, second_num: f32) -> f32 {

    first_num / second_num
}

fn pow_nums(first_num: f32, second_num: f32) -> f32 {

    first_num.powf(second_num)
}

fn mod_nums(first_num: f32, second_num: f32) -> f32 {

    first_num % second_num
}