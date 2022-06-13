mod functions;
use std::{path::Path, fs};
use serde::{Deserialize, Serialize};
use serde_json;
use webbrowser;

#[derive (Deserialize, Serialize)]
pub struct UserCode {
    pub password : Vec<u8>,
    pub recovery : (u8, Vec<u8>),
    pub weblinks : Vec<String>,
    pub alg : [u8; 3]}

const CREDIT : &str = "Created by Bhop, 2022.";

fn main() {
    functions::newscreen(CREDIT);
    println!("NOTE: It is recommended that your machine only runs one instance of this software");
    println!("      at a time for the best experience\n");
    functions::escape();
    
    const RECOVERY_QUESTIONS : [&str ; 5] = ["What is the name of your favourite band or singer?",
        "What is your favourite colour?",
        "What was/ is your favourite subject in school?",
        "What nickname did/ do you have?",
        "What is the name of your favourite pet?"];
    
    //OPENING SEQUENCE: REGISTRATION/ RESET/ LOGIN
    let user_data_exists : bool = Path::new("./dt.json").exists();
    if !user_data_exists {
        let alg : [u8; 3] = functions::generate_cartesian_alg();
        let weblinks : Vec<String> = Vec::from([functions::encrypt_affine2(&"https://www.google.com".to_string(), &alg)]); //google's a constant
        loop {
            let password : String = register_user_password();
            let recovery_answer : (u8, String) = register_user_recovery(RECOVERY_QUESTIONS);
            if confirm_user_input(&["password", "recovery answer"].to_vec(), &[password.clone(), recovery_answer.1.clone()].to_vec()) {
                let password_final : Vec<u8> = functions::hash(&password);
                let recovery_final : (u8, Vec<u8>) = (recovery_answer.0, functions::hash(&recovery_answer.1.to_lowercase()));
                let dt = UserCode {
                    password: password_final, recovery: recovery_final,
                    weblinks: weblinks, alg: alg};
                //now we dump it:
                let json_str : String = serde_json::to_string(&dt).unwrap();
                fs::write("./dt.json", json_str).expect("Unable to write to file");
                break;
            }
        }
        functions::newscreen(CREDIT);
        println!("  Success! You can now continue :)");
        functions::escape();
    }

    else {
        let data : String = fs::read_to_string("./dt.json").expect("Unable to read file");
        let mut user_details : UserCode = serde_json::from_str(data.as_str()).unwrap();
        let ans : String = login(user_details.password);
        if ans == String::from("FORGOT") {
            let new_password = reset_password(RECOVERY_QUESTIONS, user_details.recovery.clone());
            user_details.password = functions::hash(&new_password);

            let json_str : String = serde_json::to_string(&user_details).unwrap();
            fs::write("./dt.json", json_str).expect("Unable to write to file");
            functions::newscreen(CREDIT);
            println!("password updated!");
            functions::escape();
        }
    }
    //OPENING SEQUENCE *END*
    loop {
        let data : String = fs::read_to_string("./dt.json").expect("Unable to read file");
        let mut user_details : UserCode = serde_json::from_str(data.as_str()).unwrap();
        let mut user_input : String = String::from("");
        while (user_input != "e".to_string()) && (user_input != "a".to_string()) && (user_input != "r".to_string()) {
            functions::newscreen(CREDIT);
            println!("Select one of the following websites, or use a command:");
            print!("      'E' => exit; 'A' => add site");
            if user_details.weblinks.len() != 1 {print!("; 'R' => remove site\n\n");}  
            else {println!("\n");}      
            for x in 0..user_details.weblinks.len() {
                println!(" {}) {}", (x+1), functions::decrypt_affine2(&user_details.weblinks[x], &user_details.alg) );
            }
            println!("\nNOTE: erroneous inputs will be ignored");
            println!("input: ");
            user_input = functions::get_string(true).to_lowercase();
            if user_input.parse::<u32>().is_ok() {
                let user_int : u32 = user_input.parse::<u32>().unwrap();
                if (user_int >= 1) && (user_int <= (user_details.weblinks.len()).try_into().unwrap()) {
                    let url : String = functions::decrypt_affine2(&user_details.weblinks[user_int as usize-1], &user_details.alg);
                    webbrowser::open(&url).unwrap();}
            }
        }
        if user_input == "e".to_string() {break;}
        if user_input == "a".to_string() {
            loop {
                let mut new_weblink : String = String::from("");
                while (new_weblink.starts_with("https://") == false) && (new_weblink.starts_with("http://") == false) {
                    functions::newscreen(CREDIT);
                    println!("please paste the url you would like to add below:");
                    println!("  (NOTE: entered websites must begin with https://... OR http://...)");
                    println!("  (NOTE: press 'B' to return to the homepage)");
                    new_weblink = functions::get_string(true); //cant make this to_lc
                    if (new_weblink == "b".to_string()) || (new_weblink == "B".to_string()) {break;}
                }
                if (new_weblink == "b".to_string()) || (new_weblink == "B".to_string()) {break;}

                if confirm_user_input(&["add"].to_vec(),&[new_weblink.clone()].to_vec()) {
                    if !user_details.weblinks.contains(&functions::encrypt_affine2(&new_weblink, &user_details.alg)) {
                        user_details.weblinks.push(functions::encrypt_affine2(&new_weblink, &user_details.alg));
                        let json_str : String = serde_json::to_string(&user_details).unwrap(); //no dupes basically
                        fs::write("./dt.json", json_str).expect("Unable to write to file");
                    }
                    break;
                }
            }
        }
        if (user_input == "r".to_string()) && (user_details.weblinks.len() != 1) {
            loop {            
                let mut valid_input : bool = false;
                let mut user_input : String = String::from("");
                let mut selected_weblink : usize = 0;
                functions::newscreen(CREDIT);
                while !valid_input {
                    println!("What website would you like to remove?\n");
                    for x in 1..user_details.weblinks.len() {
                        println!("  {}) {}", x, functions::decrypt_affine2(&user_details.weblinks[x], &user_details.alg));
                    }
                    println!("\n(NOTE: press 'B' to return to the homepage)");
                    user_input = functions::get_string(true).to_lowercase();
                    if user_input == "b".to_string() {break;}
                    if user_input.parse::<u32>().is_ok() {
                        selected_weblink = user_input.parse::<usize>().unwrap();
                        if (selected_weblink >= 1) && (selected_weblink <= user_details.weblinks.len()-1) {valid_input = true;}
                        else {functions::newscreen("WARNING: INPUT OUT OF RANGE");}
                    }
                    else {functions::newscreen("WARNING: INVALID INPUT");}
                }
                if user_input == "b".to_string() {break;}

                let decrypted_link : String = functions::decrypt_affine2(&user_details.weblinks[selected_weblink], &user_details.alg); //easier to read lol
                if confirm_user_input(&["remove"].to_vec(), &[decrypted_link].to_vec()) {
                    user_details.weblinks.remove(selected_weblink);
                    let json_str : String = serde_json::to_string(&user_details).unwrap();
                    fs::write("./dt.json", json_str).expect("Unable to write to file");
                    break;
                }
            }
        }
    }
}

//OPENING SCREENS
fn register_user_password() -> String {
    functions::newscreen(CREDIT);
    let mut password : String = String::from("");
    let mut password_confirm : String;
    let mut valid_password : bool = false;
    while !valid_password {
        println!("Please create a password: ");
        println!("  (at least 5 characters, and no spaces allowed.)");
        password = functions::get_string(true);
        println!("Confirm your password: ");
        password_confirm = functions::get_string(true);
        if password != password_confirm {functions::newscreen("WARNING: Passwords didn't match");}
        else if !functions::validate_password(&password) {functions::newscreen("WARNING: Invalid Password. Try Again");}
        else {valid_password = true;}
    }
    return password; //the program is pretty inconsistent to not have confirmation here, but in other fns like reset_password...
}

fn register_user_recovery(questions : [&str ; 5]) -> (u8, String) {
    functions::newscreen(CREDIT);
    let mut chosen_question : u8;
    loop {
        println!("If you ever forget your password, you'll be able to reset it by");
        println!("answering one of these questions:\n");
        for x in 0..questions.len() {
            println!("  {}) {}", x+1, questions[x]);
        }
        println!("\nWhich question would you like to answer? (e.g. 1,2,3...)");
        let user_input : String = functions::get_string(true);
        if !user_input.parse::<u8>().is_ok() {functions::newscreen("WARNING: Invalid input entered");}
        else {
            chosen_question = user_input.parse::<u8>().unwrap();
            if (chosen_question >= 1) && (chosen_question <= 5) {chosen_question = chosen_question - 1; break}
            else {functions::newscreen("WARNING: Input out of range");}
        }
    }
    let mut user_answer : String = String::from("");
    while user_answer == String::from("") {
    functions::newscreen(CREDIT);
    println!("Ok: {}", questions[chosen_question as usize]);
    user_answer = functions::get_string(true);
    }
    return (chosen_question, user_answer);
}

fn login(hashed_pword : Vec<u8>) -> String {
    functions::newscreen(CREDIT);
    loop {
        println!("Password: (enter 'F' if you forgot/ would like to reset your password)");
        let entered_password : String = functions::get_string(true);
        if entered_password.to_lowercase() == String::from("f") {return "FORGOT".to_string();}
        let entered_final : Vec<u8> = functions::hash(&entered_password);
        if hashed_pword == entered_final {break;}
        functions::newscreen("INCORRECT PASSWORD: Try again.");       
    }
    return "ACCEPTED".to_string(); //basically an ignore case
}

fn reset_password(questions : [&str ; 5], rec_answer : (u8, Vec<u8>)) -> String {
    functions::newscreen(CREDIT);
    println!("(If you would like to go back to login, Please restart the program)");
    loop {
        println!("{}", questions[rec_answer.0 as usize]);
        let entered_answer : String = functions::get_string(true).to_lowercase();       
        if rec_answer.1 == functions::hash(&entered_answer) {break;}
        functions::newscreen(CREDIT);
        println!("INCORRECT ANSWER: Try again.");        
    }

    let mut password : String;
    let mut password_confirm : String;
    functions::newscreen(CREDIT);
    println!("PASSWORD RESET:");
    loop {
        println!("Please enter your new password:");
        password = functions::get_string(true);
        println!("Confirm password:");
        password_confirm = functions::get_string(true);
        functions::newscreen(CREDIT);
        if password != password_confirm {println!("WARNING: Passwords don't match. Try Again");}
        else if !functions::validate_password(&password) {println!("WARNING: Invalid Password entered. Try Again");}
        else {
            if confirm_user_input(&["password"].to_vec(), &[password.clone()].to_vec()) {break;}
            else {functions::newscreen(CREDIT);}
        }
    }
    return password;
}

//VALIDATION (i.e. all y/N operations) - this should be in functions.rs, but idc!
fn confirm_user_input(info_types : &Vec<&str>, details : &Vec<String>) -> bool {
    if info_types.len() != details.len() {return false} //fail-state
    let mut confirmation : String = String::from("");
    while confirmation != "y".to_string() && confirmation != "n".to_string() {
        functions::newscreen(CREDIT);
        for x in 0..info_types.len() {
            println!("{} : {}", info_types[x], details[x]);}
        println!("\nare these details ok? (y/N)");
        confirmation = functions::get_string(true).to_lowercase();
    }
    if confirmation == "y".to_string() {return true;}
    return false;
}