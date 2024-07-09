use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for (index, argument) in args.iter().enumerate() {
        println!("MESSAGE_{}: '{}',",camel_to_snake(argument), argument);
    }
}

fn camel_to_snake(input: &str) -> String {
    let re = Regex::new(r"([A-Z]+)([A-Z][a-z])|([a-z\d])([A-Z])").unwrap();
    let snake = re.replace_all(input, |caps: &regex::Captures| {
        if let Some(m) = caps.get(1) {
            format!("{}{}", m.as_str().to_lowercase(), caps[2].to_lowercase())
        } else {
            format!("{}_{}", &caps[3], &caps[4].to_lowercase())
        }
    }).to_uppercase();
    snake.to_string()
}
