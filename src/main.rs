use std::io;
use std::fs;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::process::Command;



fn main() {

    Command::new("clear")
        .spawn()
        .unwrap()
        .wait();
    
    println!("Xlaunch v1.0");

    #[derive(Clone, Serialize, Deserialize, Debug)] 
    struct Entry {
        name: String,
        path: String,
        wine: bool,
    }

    let mut entries = Vec::new();
    entries.push(Entry { name: "i".to_string(), path: "o".to_string(), wine: false });

    let mut data = fs::read_to_string("data.json").expect("unable to read file");
    entries = serde_json::from_str(&data).unwrap();


    loop{
        let mut choice = String::new();
        println!("1: create a new entry");
        println!("2: delete an entry");
        println!("3: lauch a software/game");
        println!("4: exit");
        println!("Please input your choice.");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You choosed: {}", choice);
        
        if choice == 1 || choice == 2 || choice == 3 || choice == 4{
            println!("valid entry.");
        }else{
            println!("invalid entry please retry.");
        };
        

        if choice == 1 {

            println!("listing existing entries...");
            for i in 0..entries.len() {
                println!("{}",entries[i].name);
            };

            println!("entry name? c to cancel");
            let mut entry_name = String::new();
            io::stdin()
                .read_line(&mut entry_name)
                .expect("Failed to read line");
            
            if entry_name.trim().to_string() == "c".to_string() {
                println!("cancelling...");
                continue;
            }

            println!("path to entry?");
            let mut entry_path = String::new();
            io::stdin()
                .read_line(&mut entry_path)
                .expect("Failed to read line");
            
            let mut entry_type = String::new();
            entry_type = entry_path[entry_path.len() - 4..entry_path.len()].trim().to_string();
            if entry_type.to_string() == "exe".to_string() {
                println!("autodetect detected that wine is necessary to launch this software/game, adding wine: True to entry.");
                entries.push(Entry { name: entry_name.trim().to_string(), path: entry_path.trim().to_string(), wine: true });
            }else{
                println!("autodetect detected that wine is unnecessary to launch this software/game, adding wine: False to entry.");
                entries.push(Entry { name: entry_name.trim().to_string(), path: entry_path.trim().to_string(), wine: false });
            }
        }

        if choice == 2 {

            println!("listing existing entries...");
            for i in 0..entries.len() {
                println!("{}",entries[i].name);
            };
            println!("which entry do you want to delete? c to cancel");
            let mut entry_to_delete = String::new();
            io::stdin()
                .read_line(&mut entry_to_delete)
                .expect("Failed to read line");

            if entry_to_delete.trim().to_string() == "c".to_string() {
                println!("cancelling...");
                continue;
            }

            println!("deleting {}",entry_to_delete);
            for i in 0..entries.len() {
                if entries[i].name.to_string() == entry_to_delete.trim().to_string(){
                    for j in i..entries.len()-1{
                        entries[j] = entries[j+1].clone();
                    };
                    entries.truncate(entries.len()-1);
                    break;
                }
            };
        }

        if choice == 3 {
            println!("listing existing entries...");
            for i in 0..entries.len() {
                println!("{}",entries[i].name);
            };
            println!("which entry do you want to launch?");
            let mut entry_to_launch = String::new();
            io::stdin()
                .read_line(&mut entry_to_launch)
                .expect("Failed to read line");
            println!("launching {}",entry_to_launch);
            for i in 0..entries.len() {
                if entries[i].name.to_string() == entry_to_launch.trim().to_string(){
                    if entries[i].wine == false {
                        let shell = include_str!("launch.sh");
                        let rust_var = entries[i].path.clone();
                        let script = format!("VARIABLE={} ; {}", rust_var, shell);
                        std::process::Command::new("sh")
                            .arg("-c")
                            .arg(script)
                            .spawn()
                            .unwrap()
                            .wait();
                    }else{
                        let shell = include_str!("launch_wine.sh");
                        let rust_var = entries[i].path.clone();
                        let script = format!("VARIABLE={} ; {}", rust_var, shell);
                        std::process::Command::new("sh")
                            .arg("-c")
                            .arg(script)
                            .spawn()
                            .unwrap()
                            .wait();  
                    }
                }
            };


        }

        if choice == 4 {
            println!("exiting...");
            let mut serialized_entries =serde_json::to_string(&entries).unwrap();
            fs::write("data.json", serialized_entries).expect("unable to write file");

            break;
        }
    };
}