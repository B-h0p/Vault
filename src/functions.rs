use std::io;
use clearscreen;
use rand::Rng;
use sha2::{Sha256, Digest};

pub fn get_string(remove_opening_spaces : bool) -> String { //general function
    let mut output : String = String::new();
    let reader = io::stdin();
    reader.read_line(&mut output)
        .expect("Failed to read line");
           
    while (output.ends_with("\n")) || (output.ends_with("\r")) || (output.ends_with(" ")) {output.pop();}       
    while ((output.starts_with("\n")) || (output.starts_with("\r")) || (output.starts_with(" "))) && remove_opening_spaces {
        output.remove(0);}
    return output;
}

pub fn escape() { //general function
    let mut cont : String = String::new();     
    println!("Please press ENTER to continue:");
    let reader = io::stdin();
    reader.read_line(&mut cont)
        .expect("Failed to read line"); 
}

pub fn newscreen(msg : &str) {
    clearscreen::clear().unwrap();
    println!("{}\n", msg);
}

pub fn generate_cartesian_alg() -> [u8; 3] {
    let mut function : [u8; 3] = [0,0,0];
    function[0] = rand::thread_rng().gen_range(1..=100);
    function[1] = rand::thread_rng().gen_range(1..=25); 
    function[2] = rand::thread_rng().gen_range(1..=5);  
    return function;
}

pub fn validate_password(pword : &String) -> bool {
    if pword.len() < 5 {return false;}
    let pword_test : Vec<char> = pword.chars().collect();
    for x in pword_test {
        if x == ' ' {return false;}}
    return true;
}

pub fn hash(field : &String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(&field);
    let raw_hash = hasher.finalize();
    let mut result : Vec<u8> = Vec::from([]);
    for x in raw_hash {result.push(x);}
    return result;
}

pub fn encrypt_affine2(msg : &String, key : &[u8; 3]) -> String {
    const UPPERCASE_ASCII : u32 = 65;
    const LOWERCASE_ASCII : u32 = 97;
    let msg_list : Vec<char> = msg.chars().collect();
    let mut final_url : String = String::from("");
    let mut increment : u32 = 1;
    for x in 0..msg_list.len() {
        if (((msg_list[x] as u8) >= 97)&&((msg_list[x] as u8) <= 122)) || ((msg_list[x] as u8) >= 65)&&((msg_list[x] as u8) <= 90) {
            let mut case : u32 = LOWERCASE_ASCII;
            if (msg_list[x] as u8) <= 90 {case = UPPERCASE_ASCII;}
            let mut new_sum : u32 = 0;
            for y in 0..key.len() {
                new_sum = new_sum + ((key[y] as u32) * (u32::pow(increment, y as u32)))
            }
            let updated_char : char = ((((msg_list[x] as u32) + new_sum - case) % 26) + case).try_into().unwrap();
            final_url.push(updated_char);
            increment = increment + 1;
        }
        else {final_url.push(msg_list[x]);}
    }
    return final_url;
}

pub fn decrypt_affine2(msg : &String, key : &[u8; 3]) -> String {
    const UPPERCASE_ASCII : u32 = 65;
    const LOWERCASE_ASCII : u32 = 97;
    let msg_list : Vec<char> = msg.chars().collect();
    let mut final_url : String = String::from("");
    let mut increment : u32 = 1;
    for x in 0..msg_list.len() {
        if (((msg_list[x] as u8) >= 97)&&((msg_list[x] as u8) <= 122)) || ((msg_list[x] as u8) >= 65)&&((msg_list[x] as u8) <= 90) {
            let mut case : u32 = LOWERCASE_ASCII;
            if (msg_list[x] as u8) <= 90 {case = UPPERCASE_ASCII;}
            let mut new_sum : u32 = 0;
            for y in 0..key.len() {
                new_sum = new_sum + ((key[y] as u32) * (u32::pow(increment, y as u32)))
            }
            let updated_value_1 : i32 = (msg_list[x] as i32) - new_sum as i32 - (case as i32);
            let updated_value_2 : u32;
            if updated_value_1 >= 0 {updated_value_2 = updated_value_1 as u32 % 26}
            else {
                let temp = 0 - updated_value_1; //updated_value_1 is now positive
                updated_value_2 = (26 - (temp as u32 % 26)) % 26;
            }
            let updated_char : char = (updated_value_2 + case).try_into().unwrap();
            final_url.push(updated_char);
            increment = increment + 1;
        }
        else {final_url.push(msg_list[x]);}
    }
    return final_url;} //TODO - improve control flow, variable name control on main