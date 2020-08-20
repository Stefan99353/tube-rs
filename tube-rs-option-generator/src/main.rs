use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

fn main() {
    let mut flags: Vec<YoutubeFlag> = vec![];

    loop {
        println!("What do you want to do?");
        println!(" [1] New entry");
        println!(" [2] Write json");
        println!(" [3] Load json");
        println!(" [4] Write SQL");
        println!(" [q] Quit");

        let mut act: String = String::new();
        std::io::stdin().read_line(&mut act).unwrap();
        trim_newline(&mut act);

        let action: Action;

        if act == String::from("1") {
            action = Action::New;
        } else if act == String::from("2") {
            action = Action::Write;
        } else if act == String::from("3") {
            action = Action::Load;
        } else if act == String::from("4") {
            action = Action::SQL
        } else {
            action = Action::Quit;
        }

        match action {
            Action::New => {
                let flag = add_flag();
                flags.push(flag);
            }
            Action::Write => {
                write_file(&flags);
                exit(0);
            }
            Action::Load => {
                flags = load_file();
            }
            Action::SQL => write_sql(&flags),
            Action::Quit => {
                exit(0);
            }
        }
    }
}

fn write_sql(flags: &Vec<YoutubeFlag>) {
    let mut file_up = File::create("up.sql").unwrap();

    let mut flag_id: u32 = 1;
    let mut value_id: u32 = 1;
    for flag in flags {
        // Write Flag
        file_up.write(format!("INSERT INTO youtube_flags (id, name, description, flag) VALUES ({},'{}','{}','{}')", flag_id, flag.text, flag.description, flag.flag).as_bytes()).unwrap();
        file_up.write("\n".as_bytes()).unwrap();

        // Write FlagValue
        for flag_value in &flag.flag_values {
            file_up.write(format!("INSERT INTO flag_values (id, name, value, youtube_flag_id) VALUES ({},'{}','{}',{})", value_id, flag_value.name, flag_value.value, flag_id).as_bytes()).unwrap();
            file_up.write("\n".as_bytes()).unwrap();
            value_id += 1;
        }
        flag_id += 1;
    }
}

fn write_file(flags: &Vec<YoutubeFlag>) {
    let json_string = serde_json::to_string(&flags).unwrap();

    let mut file = File::create("data.json").unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

    println!("Written to 'data.json'");
}

fn load_file() -> Vec<YoutubeFlag> {
    let mut file = File::open("data.json").unwrap();
    let mut json_string = String::new();

    file.read_to_string(&mut json_string).unwrap();

    serde_json::from_str(json_string.as_str()).unwrap()
}

enum Action {
    New,
    Write,
    Load,
    SQL,
    Quit,
}

fn add_flag() -> YoutubeFlag {
    let mut text = String::new();
    let mut description = String::new();
    let mut flag = String::new();
    let mut custom = String::new();

    // Text
    println!("Text: ");
    std::io::stdin().read_line(&mut text).unwrap();
    // Description
    println!("Description: ");
    std::io::stdin().read_line(&mut description).unwrap();
    // Flag
    println!("Flag: ");
    std::io::stdin().read_line(&mut flag).unwrap();
    // Custom
    println!("Allow custom value (y/n): ");
    std::io::stdin().read_line(&mut custom).unwrap();

    let mut another: bool = true;
    let mut values: Vec<FlagValue> = vec![];

    while another {
        println!("Add value? (y/n): ");
        let mut anoth = String::new();
        std::io::stdin().read_line(&mut anoth).unwrap();
        trim_newline(&mut anoth);

        if anoth != String::from("y") {
            another = false;
        } else {
            values.push(add_value());
        }
    }

    trim_newline(&mut text);
    trim_newline(&mut description);
    trim_newline(&mut flag);
    trim_newline(&mut custom);

    let mut custom_value: bool = false;
    if custom == String::from("y") {
        custom_value = true;
    }

    let flag = YoutubeFlag {
        text,
        description,
        flag,
        custom: custom_value,
        flag_values: values,
    };

    flag
}

fn add_value() -> FlagValue {
    let mut name = String::new();
    let mut value = String::new();

    println!("Possible Value");

    // Name
    println!("Name: ");
    std::io::stdin().read_line(&mut name).unwrap();

    // Value
    println!("Value: ");
    std::io::stdin().read_line(&mut value).unwrap();

    trim_newline(&mut name);
    trim_newline(&mut value);
    FlagValue { name, value }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct YoutubeFlag {
    text: String,
    description: String,
    flag: String,
    custom: bool,
    flag_values: Vec<FlagValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FlagValue {
    name: String,
    value: String,
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
