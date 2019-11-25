use std::io;
use std::collections::HashMap;

fn count_sock_pairs(colors: Vec<i32>) -> i32 {
    let mut hmap = colors.into_iter().fold(HashMap::new(), |mut acc, curr| {
        {
            let counter = acc.entry(curr).or_insert(0);
            *counter += 1;
        }
        // println!("{:?}", acc);
        acc
    });
        
    let mut pairs = 0;
    
    for (_, v) in hmap.into_iter() {                
       pairs += v / 2         
    };

    pairs
}


fn main() {
    let num1: &String = &from_stdin();
    let _second_param: i32 = parse_single_val(num1); // unneeded
    let sock_colors_str: &String = &from_stdin();
    let sock_color_values: Vec<i32> = parse_multi_val(sock_colors_str);
    let answer = count_sock_pairs(sock_color_values);

    println!("{}", answer)
}

fn parse_single_val(string: &String) -> i32 {
    std::str::FromStr::from_str(string.trim()).unwrap()
}

fn parse_multi_val(string: &String) -> Vec<i32> {
    let arr = string.split(" ").collect::<Vec<&str>>();
    arr.iter().map(|x| std::str::FromStr::from_str(x).unwrap()).collect()
}

fn from_stdin() -> std::string::String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}
