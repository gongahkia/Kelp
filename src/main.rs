    // DEBUG
        // -- add a check for editing tasks and sorting tasks by tag number, if number inputted
        // exceeds the available index range, reject and ask for a new number (index errors)
    // IMPLEMENT
        // -- add option to delete project folders when projects are completed
        // -- Add seperate recycle_bin_vector that stores deleted or completed tasks for PROJECTS
        // until the entire project is completed, store this vector in another list

// external crate imports
extern crate colored;
extern crate chrono;
extern crate substring;

// required imports
use std::io;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::fs::File;
use std::io::Write;
use colored::*;
use std::{thread, time};
use chrono::Local;
use std::process::Command;

#[derive(Debug)]
struct Task {
    task_name:String,
    task_description:String,
    task_deadline:[i32; 3],
    task_urgency:UrgencyLevel,
    task_tags:String,
}

fn display_task_vector(storage_vector:&Vec<Task>, counter:u8) {
    let mut counter = counter;
    for task in storage_vector {
        println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
        println!("{} {}", "Name: ".yellow(), task.task_name);
        println!("{} {}", "Description: ".yellow(), task.task_description);
        let mut task_deadline_string:String = String::from("");
        for component in task.task_deadline {
            task_deadline_string.push_str(component.to_string().as_str());
            task_deadline_string.push_str("/");
        };
        task_deadline_string.pop();
        println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
        if task.task_tags == String::from(" ") {
            println!("{} {}\n", "Urgency: ".yellow(), task.task_urgency);
        } else {
            let task_tags_collection:Vec<&str> = task.task_tags.split("&").collect();
            let mut task_tags_for_reader:String = String::new();
            for item in task_tags_collection {
                task_tags_for_reader.push_str(item);
                task_tags_for_reader.push_str(", ");
            }
            task_tags_for_reader.pop();
            task_tags_for_reader.pop();
            println!("{} {}", "Urgency: ".yellow(), task.task_urgency);
            println!("{} {}\n", "Tags: ".yellow(), task_tags_for_reader);
        }
        counter += 1;
    }
}

fn edit_task_name(index:usize, mut storage_vector:Vec<Task>) -> Vec<Task> {
    println!("{}", "Enter the new task name:".yellow());
    let mut new_task_name:String = String::new();
    io::stdin().read_line(&mut new_task_name).expect("Failed to read line");
    let new_task_name_string:String = new_task_name.as_str().trim_end().to_string();
    storage_vector[index].task_name = new_task_name_string;    
    // return value
    storage_vector
}

fn edit_task_description(index:usize, mut storage_vector:Vec<Task>) -> Vec<Task> {
    println!("{}", "Enter the new task description:".yellow());
    let mut new_task_description:String = String::new();
    io::stdin().read_line(&mut new_task_description).expect("Failed to read line");
    let new_task_description_string:String = new_task_description.as_str().trim_end().to_string();
    storage_vector[index].task_description = new_task_description_string;
    storage_vector
}

fn edit_task_deadline(index:usize, mut storage_vector:Vec<Task>) -> Vec<Task> {
    println!("{}", "Enter the new task deadline:".yellow());

    loop {
        let mut new_userinput_task_deadline_raw:String = String::new();
        io::stdin().read_line(&mut new_userinput_task_deadline_raw).expect("Failed to read line");
        let new_userinput_task_deadline_raw_array = new_userinput_task_deadline_raw.split("/");
        let new_userinput_task_deadline_array: Vec<&str> = new_userinput_task_deadline_raw_array.collect();
        
        // checking for valid number of fields input (characters, str literals and numbers covered)
        if new_userinput_task_deadline_array.len() != 3 {
            println!("{}\nEnter {} in the following format {}: ", "Invalid input detected.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
            continue;
        }

        // checking for characters instead of date input if there are 3 fields
        if new_userinput_task_deadline_array[0].chars().all(char::is_numeric) && new_userinput_task_deadline_array[1].chars().all(char::is_numeric) && new_userinput_task_deadline_array[2].trim_end().chars().all(char::is_numeric) {
        } else {
            println!("{}\nEnter {} in the following format {}: ", "Enter a valid integer input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
            continue;
        }

        // these have to be signed integers first, to allow for subsequent error checking
        let new_userinput_task_deadline_day_int:i32 = new_userinput_task_deadline_array[0].trim_end().parse().expect("Failed to parse number");
        let new_userinput_task_deadline_month_int:i32 = new_userinput_task_deadline_array[1].trim_end().parse().expect("Failed to parse number");
        let new_userinput_task_deadline_year_int:i32 = new_userinput_task_deadline_array[2].trim_end().parse().expect("Failed to parse number");
        
        // checking for valid date inputs
        if new_userinput_task_deadline_day_int > 31 || new_userinput_task_deadline_day_int < 1 {
            println!("{}\nEnter {} in the following format {}: ", "Enter a valid day input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
            continue;
        }
        if new_userinput_task_deadline_month_int > 12 || new_userinput_task_deadline_month_int < 1 {
            println!("{}\nEnter {} in the following format {}: ", "Enter a valid month input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
            continue;
        } 
        if new_userinput_task_deadline_year_int < 23 || new_userinput_task_deadline_year_int > 99 {
            println!("{}\nEnter {} in the following format {}: ", "Enter a valid year input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
            continue; 
        }
    storage_vector[index].task_deadline = [new_userinput_task_deadline_day_int, new_userinput_task_deadline_month_int, new_userinput_task_deadline_year_int];
    break
    }
    storage_vector
}

fn edit_task_urgency(index:usize, mut storage_vector:Vec<Task>) -> Vec<Task> {
    println!("{}", "Enter the new task urgency:".yellow());
    let new_task_urgency:UrgencyLevel;
    loop {
        let mut new_userinput_task_urgency_string:String = String::new();
        io::stdin().read_line(&mut new_userinput_task_urgency_string).expect("Failed to read line");
        let new_userinput_task_urgency_stringliteral:&str = new_userinput_task_urgency_string.as_str().trim_end();
        match new_userinput_task_urgency_stringliteral {
            "l" => {
                new_task_urgency = UrgencyLevel::Low;
                break;
            },
            "m" => {
                new_task_urgency = UrgencyLevel::Medium;
                break;
            },
            "h" => {
                new_task_urgency = UrgencyLevel::High;
                break;
            },
            &_ => {
                println!("{} [L/M/H]: ", "Please enter a valid input!".red().underline());
                }
            }
        }
    storage_vector[index].task_urgency = new_task_urgency;
    storage_vector
}

fn edit_task_tags(index:usize, mut storage_vector:Vec<Task>) -> Vec<Task> {
    println!("{}", "Enter new task tags, separated by a space:".yellow());
    let mut new_task_tags:String = String::new();
    io::stdin().read_line(&mut new_task_tags).expect("Failed to read line");
    let new_task_tags_string:String = new_task_tags.as_str().trim_end().to_string();
    let new_task_tag_collection:Vec<&str> = new_task_tags_string.split(" ").collect();
    let new_task_tag_reformatted:String = new_task_tag_collection.join("&");
    storage_vector[index].task_tags = new_task_tag_reformatted;
    storage_vector
}

#[derive(Debug, Clone, Copy)]
enum UrgencyLevel {
    Low,
    Medium,
    High,
}

// converting enum to string 
impl fmt::Display for UrgencyLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UrgencyLevel::Low => write!(f, "Low"),
            UrgencyLevel::Medium => write!(f, "Medium"),
            UrgencyLevel::High => write!(f, "High"),
        }
    }
}

// converting string to enum 
impl FromStr for UrgencyLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<UrgencyLevel, ()> {
        match s {
            "Low" => Ok(UrgencyLevel::Low),
            "Medium" => Ok(UrgencyLevel::Medium),
            "High" => Ok(UrgencyLevel::High),
            _ => Err(()),
        }
    }
}

fn main() {

    Command::new("clear").status().expect("Failed to read command");
    println!("{} {}\n", "Welcome to".yellow(), "Kelp 1.0".cyan());
    println!("{}", "What would you like to do today?".yellow());
    println!("{}\n{}\n", "[P]roject Management".bright_green(), "[T]ask management".magenta());
    let mut what_to_do:String = String::new();
    io::stdin().read_line(&mut what_to_do).expect("Failed to read line");
    let what_to_do_formatted:&str = what_to_do.trim_end();

    match what_to_do_formatted {

        "p" => {
            // println!("Project management time");
            let is_file_directory:bool = Path::new(".kelpProjects").is_dir();
            if is_file_directory {
                // (1) file directory exists
                // println!("Do something with it");
                let number_of_files:usize = fs::read_dir(".kelpProjects").expect("Failed to read directory").count();
                if number_of_files > 0 {
                    // (1.1) directory is not empty
                    // ADD CODE HERE IN LINE 224
                    Command::new("clear").status().expect("Failed to run command");
                    let mut project_do:String = String::new();
                    println!("{}", "What would you like to do with your projects?".yellow());
                    println!("{}", "[O]pen existing projects".magenta());
                    println!("{}", "[C]reate new project".cyan());
                    io::stdin().read_line(&mut project_do).expect("Failed to read line");
                    let project_do_formatted:&str = project_do.trim_end();

                    match project_do_formatted {
                        "o" => (), // there is no automatic break in a match statement
                        "c" => {
                            // CODE THAT CREATES NEW PROJECT DIRECTORIES INSIDE THE FILE PATH
                            Command::new("clear").status().expect("Failed to run command");
                            let mut project_name:String = String::new();
                            println!("{} {}", "Input".cyan(), "new project name:".cyan().underline());
                            io::stdin().read_line(&mut project_name).expect("Failed to read line");
                            let project_name_string:String = project_name.trim_end().to_string();
                            Command::new("clear").status().expect("Failed to run command");
                            println!("{} {}{}{}", "Starting up".yellow(), "new project ".yellow(), project_name_string.cyan().underline(), "...".yellow());
                            let mut target_file_path:String = String::from(".kelpProjects/");
                            target_file_path.push_str(project_name_string.as_str());
                            fs::create_dir(target_file_path).expect("Unable to create the desired file!");
                            thread::sleep(time::Duration::from_secs(2));
                        },
                        _ => {
                            Command::new("clear").status().expect("Failed to read line");
                            println!("{}", "Invalid input detected.".red().underline());
                            println!("{}", "Defaulting to opening your projects...".yellow());
                            thread::sleep(time::Duration::from_secs(3));
                        },
                    }
                    
                    let mut file_name_list:Vec<String> = Vec::new();
                    for file in fs::read_dir(".kelpProjects").expect("Unable to read file directory") {
                        // println!("{}", file.expect("Unable to open file").path().display());
                        let file_name:String = file.expect("Unable to open file").path().display().to_string();
                        file_name_list.push(file_name);
                    }
                    Command::new("clear").status().expect("Failed to run command.");
                    println!("{}", "File directory already exists".yellow());
                    println!("{}\n", "The following projects were found.".yellow());
                    // println!("{:?}", file_name_list);
                    let mut counter:i8 = 1;
                    for file in &file_name_list {
                        println!("{} | {}", counter, file.split("/").collect::<Vec<&str>>()[1].bright_green());
                        counter += 1;
                    }
                    let mut project_number:String = String::new();
                    println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the project you would like to work on:".yellow());
                    io::stdin().read_line(&mut project_number).expect("Failed to read line");
                    Command::new("clear").status().expect("Failed to run command");
                    let project_number_int:usize = project_number.trim_end().parse::<usize>().expect("Failed to parse number") - 1;
                    // println!("{}", project_number_int);
                    let mut target_file_path:String = file_name_list[project_number_int].to_owned();
                    target_file_path.push_str("/.kelpStorage");
                    // handle opening of folder to check whether a .kelpStorage file exists

                    // -----
                    
                    // the start of the actual task loop

                    let mut storage_vector:Vec<Task> = vec![];

                    // printing of logo 
                    Command::new("clear").status().expect("Failed to call command");

                    // println!("{}", target_file_path);

                    println!("{} {}\n", "Project name:".yellow(), target_file_path.split("/").collect::<Vec<&str>>()[1].bright_green().underline());

                    // reading of local file and parsing it into the struct Task
                    let file_contents_results = fs::read_to_string(target_file_path.clone());
                    let _file_contents = match file_contents_results {
                        Ok(string) => {
                            println!("{}\nLoading data.", "Save file found.".green().underline());
                            let file_contents_array = string.trim_end().split("\n");
                            let file_contents_vector:Vec<&str> = file_contents_array.collect();
                            for eachtask in &file_contents_vector {
                                if eachtask.chars().last().expect("Failed to find character") == ',' {
                                    let each_task_array:Vec<&str> = eachtask.split(", ").collect();
                                    let each_task_deadline_array:Vec<&str> = each_task_array[2].trim_end_matches("/").split("/").collect();
                                    let each_task_deadline:[i32;3] = [each_task_deadline_array[0].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[1].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[2].trim_end().parse().expect("Failed to parse number")];
                                    let mut each_task_urgency_unedited:String = each_task_array[3].to_string();
                                    each_task_urgency_unedited.truncate(each_task_urgency_unedited.len() -1);
                                    match each_task_urgency_unedited.parse::<UrgencyLevel>() {
                                        Ok(level) => {
                                            let each_task_urgency:UrgencyLevel = level;
                                            let the_given_task = Task {
                                                task_name: String::from(each_task_array[0]),
                                                task_description: String::from(each_task_array[1]),
                                                task_deadline: each_task_deadline,
                                                task_urgency: each_task_urgency,
                                                task_tags: String::from(" "),
                                                };
                                            storage_vector.push(the_given_task);
                                        },
                                        Err(_) => (),
                                    }
                                } else {
                                    let each_task_array:Vec<&str> = eachtask.split(", ").collect();
                                    let each_task_deadline_array:Vec<&str> = each_task_array[2].trim_end_matches("/").split("/").collect();
                                    let each_task_deadline:[i32;3] = [each_task_deadline_array[0].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[1].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[2].trim_end().parse().expect("Failed to parse number")];
                                    match each_task_array[3].parse::<UrgencyLevel>() {
                                        Ok(level) => {
                                            let each_task_urgency:UrgencyLevel = level;
                                            let the_given_task = Task {
                                                task_name: String::from(each_task_array[0]),
                                                task_description: String::from(each_task_array[1]),
                                                task_deadline: each_task_deadline,
                                                task_urgency: each_task_urgency,
                                                task_tags: String::from(each_task_array[4]),
                                                };
                                            storage_vector.push(the_given_task);
                                        },
                                        Err(_) => (),
                                    }
                                }
                            }
                            // for debugging purposes only, to be edited out in actual program
                            println!("{}\n", "Here are your tasks:".yellow());
                            display_task_vector(&storage_vector, 1);
                        },
                        Err(_) => println!("{}\n{}\n", "No save file found.".red().underline(), "Loading a fresh save.".yellow()),
                    };

                    // -----

                    // menu screen
                    println!("{}\n{}\n{}\n{}\n{}", "What would you like to do?".yellow(), "[C]reate new task".magenta(), "[E]dit a task".blue(), "[F]inish a task".cyan(), "[S]ort tasks".bright_green());
                    let mut choose_action:String = String::new();
                    io::stdin().read_line(&mut choose_action).expect("Failed to read line");
                    let choose_action_str:&str = choose_action.as_str().trim_end();

                    match choose_action_str {
                        
                        // CREATE A TASK
                        "c" => {

                            Command::new("clear").status().expect("Failed to call command");
                            // create task loop
                            loop {
                                
                                // break condition
                                println!("[E]xit / [Enter] to {}: ", "add task".bold());
                                let mut exit_condition:String = String::new();
                                io::stdin().read_line(&mut exit_condition).expect("Failed to read line");
                                let exit_condition_str:&str = exit_condition.as_str().trim_end();
                                if exit_condition_str == "e" {
                                    if storage_vector.len() > 0 {
                                        // writing of all tasks to a local file titled .kelpStorage
                                        let mut save_file = File::create(target_file_path.clone()).expect("File already exists");
                                        for eachtask in &storage_vector {
                                            let mut task_deadline_string:String = String::from("");
                                            for component in eachtask.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            let write_to_file_result = write!(save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                            match write_to_file_result {
                                                Ok(_) => (),
                                                Err(_) => (),
                                            }
                                        }
                                    } else {
                                        Command::new("clear").status().expect("Failed to call command");
                                        println!("{}\n{}", "No tasks were created.".red().underline(), "Exiting without creating save file.".yellow());
                                    }
                                    break;
                                }

                                // -----

                                // task name
                                Command::new("clear").status().expect("Failed to call command");
                                println!("{} {}{} ", "Enter".yellow(), "task name".yellow().bold(), ":".yellow());
                                let mut userinput_task_name:String = String::new();
                                io::stdin().read_line(&mut userinput_task_name).expect("Failed to read line");
                                let userinput_task_name = String::from(userinput_task_name.trim_end());

                                // -----
                                
                                // task description
                                println!("{} {}{} ", "Enter".yellow(), "task description".yellow().bold(), ":".yellow());
                                let mut userinput_task_description:String = String::new();
                                io::stdin().read_line(&mut userinput_task_description).expect("Failed to read line");
                                let userinput_task_description = String::from(userinput_task_description.trim_end());

                                // -----
                                
                                // task deadline, parsed using destructuring
                                println!("{} {} {} {}{} ", "Enter".yellow(), "task deadline".yellow().bold(), "in the following format".yellow(), "[DD/MM/YY]".underline().yellow(), ":".yellow());
                                let userinput_task_deadline_formatted:[i32; 3];

                                loop {
                                    let mut userinput_task_deadline_raw:String = String::new();
                                    io::stdin().read_line(&mut userinput_task_deadline_raw).expect("Failed to read line");
                                    let userinput_task_deadline_raw_array = userinput_task_deadline_raw.split("/");
                                    let userinput_task_deadline_array: Vec<&str> = userinput_task_deadline_raw_array.collect();
                                    
                                    // checking for valid number of fields input (characters, str literals and numbers covered)
                                    if userinput_task_deadline_array.len() != 3 {
                                        println!("{}\nEnter {} in the following format {}: ", "Invalid input detected.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                        continue;
                                    }

                                    // checking for characters instead of date input if there are 3 fields
                                    if userinput_task_deadline_array[0].chars().all(char::is_numeric) && userinput_task_deadline_array[1].chars().all(char::is_numeric) && userinput_task_deadline_array[2].trim_end().chars().all(char::is_numeric) {
                                    } else {
                                        println!("{}\nEnter {} in the following format {}: ", "Enter a valid integer input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                        continue;
                                    }

                                    // these have to be signed integers first, to allow for subsequent error checking
                                    let userinput_task_deadline_day_int:i32 = userinput_task_deadline_array[0].trim_end().parse().expect("Failed to parse number");
                                    let userinput_task_deadline_month_int:i32 = userinput_task_deadline_array[1].trim_end().parse().expect("Failed to parse number");
                                    let userinput_task_deadline_year_int:i32 = userinput_task_deadline_array[2].trim_end().parse().expect("Failed to parse number");
                                    
                                    // checking for valid date inputs
                                    if userinput_task_deadline_day_int > 31 || userinput_task_deadline_day_int < 1 {
                                        println!("{}\nEnter {} in the following format {}: ", "Enter a valid day input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                        continue;
                                    }
                                    if userinput_task_deadline_month_int > 12 || userinput_task_deadline_month_int < 1 {
                                        println!("{}\nEnter {} in the following format {}: ", "Enter a valid month input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                        continue;
                                    } 
                                    if userinput_task_deadline_year_int < 23 || userinput_task_deadline_year_int > 99 {
                                        println!("{}\nEnter {} in the following format {}: ", "Enter a valid year input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                        continue; 
                                    }
                                    userinput_task_deadline_formatted = [userinput_task_deadline_day_int, userinput_task_deadline_month_int, userinput_task_deadline_year_int];
                                    break;
                                }
                                
                                // -----

                                // task urgency, handled by an enum
                                println!("{} {} {}{} ", "Enter".yellow(), "task urgency".yellow().bold(), "[L/M/H]".yellow().underline(), ":".yellow());
                                let userinput_task_urgency:UrgencyLevel;
                                
                                loop {
                                    let mut userinput_task_urgency_string:String = String::new();
                                    io::stdin().read_line(&mut userinput_task_urgency_string).expect("Failed to read line");
                                    let userinput_task_urgency_stringliteral:&str = userinput_task_urgency_string.as_str().trim_end();
                                    match userinput_task_urgency_stringliteral {
                                        "l" => {
                                            userinput_task_urgency = UrgencyLevel::Low;
                                            break;
                                        },
                                        "m" => {
                                            userinput_task_urgency = UrgencyLevel::Medium;
                                            break;
                                        },
                                        "h" => {
                                            userinput_task_urgency = UrgencyLevel::High;
                                            break;
                                        },
                                        // match-all pattern employed for invalid input
                                        &_ => {
                                            println!("{} [L/M/H]: ", "Please enter a valid input!".red().underline());
                                            }
                                        }
                                    }
                                
                                // -----
                                
                                // task tags added to a collection that can then be iterated over
                                println!("{} {}{} {}{}", "Enter".yellow(), "task tags".yellow(), ",".yellow(), "separated by a space".yellow().underline(), ":".yellow());

                                let mut userinput_task_tag:String = String::new();
                                io::stdin().read_line(&mut userinput_task_tag).expect("Failed to read line.");
                                let userinput_task_tag_collection:Vec<&str> = userinput_task_tag.trim_end().split(" ").collect();
                                let userinput_task_tag_collection_formatted:String = userinput_task_tag_collection.join("&");

                                // -----
                                
                                // creation of an instance of the Task struct, and assignment of internal field values
                                let given_task = Task {
                                    task_name: userinput_task_name,
                                    task_description: userinput_task_description,
                                    task_deadline: userinput_task_deadline_formatted,
                                    task_urgency: userinput_task_urgency,
                                    task_tags: userinput_task_tag_collection_formatted,
                                };
                                
                                // updating of storage_vector:Vec<Task> collection
                                storage_vector.push(given_task);
                                Command::new("clear").status().expect("Failed to call command");

                                };
                                
                            if storage_vector.len() > 0 {
                                Command::new("clear").status().expect("Failed to call command");
                                println!("{}\n", "Here are your tasks: ".yellow());
                                let mut counter:i32 = 1;
                                for task in storage_vector {
                                    if task.task_tags == "" || task.task_tags == " " {
                                        println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
                                        println!("{} {}", "Name: ".yellow(), task.task_name);
                                        println!("{} {}", "Description: ".yellow(), task.task_description);
                                        let mut task_deadline_string:String = String::from("");
                                        for component in task.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        task_deadline_string.pop();
                                        println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
                                        println!("{} {}\n", "Urgency: ".yellow(), task.task_urgency);
                                    } else {
                                        println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
                                        println!("{} {}", "Name: ".yellow(), task.task_name);
                                        println!("{} {}", "Description: ".yellow(), task.task_description);
                                        let mut task_deadline_string:String = String::from("");
                                        for component in task.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        task_deadline_string.pop();
                                        println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
                                        println!("{} {}", "Urgency: ".yellow(), task.task_urgency);
                                        let task_tags_collection:Vec<&str> = task.task_tags.split("&").collect();
                                        let mut task_tags_for_reader:String = String::new();
                                        for item in task_tags_collection {
                                            task_tags_for_reader.push_str(item);
                                            task_tags_for_reader.push_str(", ");
                                        }
                                        task_tags_for_reader.pop();
                                        task_tags_for_reader.pop();
                                        println!("{} {}\n", "Tags: ".yellow(), task_tags_for_reader);
                                    }
                                        counter += 1;
                                }
                            }
                        }, 
                        
                        // EDIT A TASK
                        "e" => {
                            Command::new("clear").status().expect("Failed to call command");
                            // .unwrap() is used for error handling here
                            if storage_vector.len() > 0 {
                                println!("{}\n", "Here are your tasks: ".yellow());
                                let mut counter:u8 = 1;
                                for task in &storage_vector {
                                    println!("{}. | {:?} ", counter, task.task_name);
                                    counter += 1;
                                }
                                println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the task you would like to edit:".yellow());
                                let mut task_to_edit:String = String::new();
                                io::stdin().read_line(&mut task_to_edit).expect("Failed to read line");
                                let task_to_edit_int:usize = task_to_edit.trim_end().parse::<usize>().expect("Failed to parse integer of usize") - 1;
                                // println!("{}", task_to_edit_int);
                                // println!("Index of the task to be edited: {}", task_to_edit_int);
                                // println!("{:?}", storage_vector[task_to_edit_int].task_name);               
                                // ----- ^ for debugging purposes
                                Command::new("clear").status().expect("Failed to call command");
                                println!("{}\n{}\n{}\n{}\n{}\n{}", "Which component of the task do you want to edit?".yellow(), "[N]ame".purple(), "[D]escription".blue(), "D[E]adline".cyan(), "[U]rgency".bright_green(), "[T]ags".bright_red());
                                let mut what_to_edit:String = String::new();
                                io::stdin().read_line(&mut what_to_edit).expect("Failed to read line");
                                // could use .unwrap() for error handling above as well
                                let what_to_edit_str = what_to_edit.as_str().trim_end();
                                match what_to_edit_str {
                                    "n" => {
                                        let storage_vector = edit_task_name(task_to_edit_int, storage_vector);
                                        let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                        for eachtask in storage_vector {
                                            let mut task_deadline_string:String = String::from("");
                                            for component in eachtask.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                            match write_to_file_result {
                                                Ok(_) => (),
                                                Err(_) => (),
                                            }
                                        }
                                    },
                                    "d" => {
                                        let storage_vector = edit_task_description(task_to_edit_int, storage_vector);
                                        let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                        for eachtask in storage_vector {
                                            let mut task_deadline_string:String = String::from("");
                                            for component in eachtask.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                            match write_to_file_result {
                                                Ok(_) => (),
                                                Err(_) => (),
                                            }
                                        }
                                    },
                                    "e" => {
                                        let storage_vector = edit_task_deadline(task_to_edit_int, storage_vector);
                                        let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                        for eachtask in storage_vector {
                                            let mut task_deadline_string:String = String::from("");
                                            for component in eachtask.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                            match write_to_file_result {
                                                Ok(_) => (),
                                                Err(_) => (),
                                            }
                                        }
                                    },
                                    "u" => {
                                        let storage_vector = edit_task_urgency(task_to_edit_int, storage_vector);
                                        let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                        for eachtask in storage_vector {
                                            let mut task_deadline_string:String = String::from("");
                                            for component in eachtask.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                            match write_to_file_result {
                                                Ok(_) => (),
                                                Err(_) => (),
                                            }
                                        }
                                    },
                                    "t" => {
                                        let storage_vector = edit_task_tags(task_to_edit_int, storage_vector);
                                        let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                        for eachtask in storage_vector {
                                            let mut task_deadline_string:String = String::from("");
                                            for component in eachtask.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                            match write_to_file_result {
                                                Ok(_) => (),
                                                Err(_) => (),
                                            }
                                        }
                                    }
                                    _ => (),
                                    // match-all statement
                                };
                            } else {
                                println!("{}\n{}", "No tasks were found.".red().underline(), "Please create a task first".yellow());
                            }
                        }, 

                        // FINISH A TASK
                        "f" => {
                            Command::new("clear").status().expect("Failed to call command");
                            // .unwrap() is used for error handling here
                            if storage_vector.len() > 0 {
                                println!("{}\n", "Here are your tasks: ".yellow());
                                let mut counter:u8 = 1;
                                for task in &storage_vector {
                                    println!("{}. | {:?} ", counter, task.task_name);
                                    counter += 1;
                                }
                                println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the task you have completed:".yellow());
                                let mut completed_task:String = String::new();
                                io::stdin().read_line(&mut completed_task).expect("Failed to read line");
                                let completed_task_int:usize = completed_task.trim_end().parse::<usize>().expect("Failed to parse integer of usize") - 1;
                                // println!("{}", completed_task_int);
                                Command::new("clear").status().expect("Failed to call command");
                                let removed_task = storage_vector.remove(completed_task_int);
                                println!("{} {}", removed_task.task_name.yellow().underline(), "has been completed!".yellow());
                                println!("{}\n", "Good job! Remember to take breaks and drink enough water!".green());
                                // println!("{:?}", storage_vector);
                                if storage_vector.is_empty() {
                                    println!("{}\n{}", "No outstanding tasks left!".yellow().underline(), "Go for a run :)".green().bold());
                                    fs::remove_file(target_file_path.clone()).expect("Failed to find file");
                                    // simply removes the local file to prevent an empty vector from being saved
                                } else {
                                    let storage_vector_len:usize = storage_vector.len();
                                    if storage_vector_len == 1 {
                                        println!("{} {}\n", "You have".yellow(), "1 outstanding task.".yellow().underline());
                                    } else {
                                        let storage_vector_len_string:String = storage_vector_len.to_string();
                                        println!("{} {}{}\n", "You have".yellow(), storage_vector_len_string.yellow().underline(), " outstanding tasks.".yellow().underline());
                                    }

                                    let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                    for eachtask in storage_vector {
                                        let mut task_deadline_string:String = String::from("");
                                        for component in eachtask.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                        match write_to_file_result {
                                            Ok(_) => (),
                                            Err(_) => (),
                                        }
                                    }
                                    // creates and rewrites the local file with the updated storage_vector
                                }
                            } else {
                                println!("{}\n{}", "No tasks were found.".red().underline(), "Please create a task first".yellow());
                            }
                        },
                        
                        // SORT TASKS
                        "s" => {
                            Command::new("clear").status().expect("Failed to call command");
                            if storage_vector.len() > 0 {
                                println!("{}\n", "Here are your tasks: ".yellow());
                                let mut counter:u8 = 1;
                                for task in &storage_vector {
                                    println!("{}. | {:?} ", counter, task.task_name);
                                    counter += 1;
                                }
                                println!("\n{}\n{}\n{}\n{}", "Sort tasks by...".yellow(), "[U]rgency".purple(), "D[E]adline".cyan(), "[T]ags".green());
                                let mut sort_criteria:String = String::new();
                                io::stdin().read_line(&mut sort_criteria).expect("Failed to read line");
                                let sort_criteria_str:&str = sort_criteria.as_str().trim_end();
                                match sort_criteria_str {
                                    "u" => {
                                        let mut low_urgency_storage_vector:Vec<Task> = vec![];
                                        let mut medium_urgency_storage_vector:Vec<Task> = vec![];
                                        let mut high_urgency_storage_vector:Vec<Task> = vec![];
                                        Command::new("clear").status().expect("Failed to call command");
                                        println!("{} {}", "Sorting by".yellow(), "urgency level.".yellow().underline().bold());
                                        for task in storage_vector {
                                            match task.task_urgency {
                                                UrgencyLevel::Low => {
                                                    low_urgency_storage_vector.push(task);
                                                },
                                                UrgencyLevel::Medium => {
                                                    medium_urgency_storage_vector.push(task);
                                                },
                                                UrgencyLevel::High => {
                                                    high_urgency_storage_vector.push(task);
                                                }, 
                                                // note that there is no need for a match-all statement since an
                                                // enum neccesitates that only its enum variants can fulfill its
                                                // requirements
                                            }
                                        }

                                        counter = 1;

                                        println!("\n{}\n", "High urgency tasks".red());
                                        if high_urgency_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&high_urgency_storage_vector, counter);
                                        };
                                        
                                        println!("\n{}\n", "Medium urgency tasks".blue());
                                        if medium_urgency_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&medium_urgency_storage_vector, counter);
                                        };

                                        println!("\n{}\n", "Low urgency tasks".green());
                                        if low_urgency_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&low_urgency_storage_vector, counter);
                                        };

                                    },

                                    "e" => {
                                        Command::new("clear").status().expect("Failed to call command");
                                        let mut overdue_storage_vector:Vec<Task> = vec![];
                                        let mut today_storage_vector:Vec<Task> = vec![];
                                        let mut this_month_storage_vector:Vec<Task> = vec![];
                                        let mut next_month_and_later_storage_vector:Vec<Task> = vec![];
                                        let current_day:String = Local::now().to_string();
                                        let current_day_str:&str = current_day.as_str();
                                        let current_day_vector:Vec<&str> = current_day_str.split(" ").collect();
                                        let current_date_vector:Vec<&str> = current_day_vector[0].split("-").collect();
                                        // println!("{:?}", current_date_vector);
                                        let current_year:i32 = current_date_vector[0][2..].parse().expect("Failed to parse number");
                                        let current_month:i32 = current_date_vector[1].parse().expect("Failed to parse number");
                                        let current_day_of_date:i32 = current_date_vector[2].parse().expect("Failed to parse number");
                                        println!("{} {}", "Sorting by".yellow(), "deadline.".yellow().underline());
                                        for task in storage_vector {
                                            if task.task_deadline[2] > current_year {
                                                // deadline is a year or more later
                                                next_month_and_later_storage_vector.push(task);
                                            } else if task.task_deadline[1] > current_month {
                                                // deadline is a month or more later
                                                next_month_and_later_storage_vector.push(task);
                                            } else if task.task_deadline[0] > current_day_of_date {
                                                // deadline is a day or more later
                                                this_month_storage_vector.push(task);
                                            } else if task.task_deadline[0] == current_day_of_date {
                                                // deadline is today
                                                today_storage_vector.push(task);
                                            } else {
                                                // deadline is overdue 
                                                overdue_storage_vector.push(task);
                                            }
                                        };

                                        let counter:u8 = 1;

                                        println!("\n{}\n", "Overdue tasks".red());
                                        if overdue_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&overdue_storage_vector, counter);
                                        };

                                        println!("\n{}\n", "Tasks due today".red());
                                        if today_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&today_storage_vector, counter);
                                        };

                                        println!("\n{}\n", "Tasks due this month".blue());
                                        if this_month_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&this_month_storage_vector, counter);
                                        };

                                        println!("\n{}\n", "Tasks due next month".green());
                                        if next_month_and_later_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&next_month_and_later_storage_vector, counter);
                                        };

                                    }, 

                                    "t" => {
                                        Command::new("clear").status().expect("Failed to call command");
                                        println!("{} {}\n", "Sorting by".yellow(), "tag type.".yellow().underline());
                                        println!("{}\n", "Here are your tags:".yellow());
                                        let mut tags_collection:Vec<&str> = Vec::new();
                                        for task in &storage_vector {
                                            let indiv_task_tag:Vec<&str> = task.task_tags.split("&").collect();
                                            // println!("{:?}", indiv_task_tag);
                                            for tag in indiv_task_tag {
                                                if tag == " " {
                                                } else {
                                                    tags_collection.push(tag);
                                                }
                                            }
                                        }
                                        tags_collection.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                                        // ^ sort a vector
                                        // println!("{:?}", tags_collection);
                                        tags_collection.dedup(); 
                                        // ^ removes duplicates
                                        // println!("{:?}", tags_collection);

                                        if tags_collection.len() > 0 {
                                            let mut counter:u8 = 1;
                                            for tag in &tags_collection {
                                                println!("{}. | {:?} ", counter, tag);
                                                counter += 1;
                                                }
                                            }
                                        println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the tag you would like to sort by:".yellow());
                                        let mut tag_num_to_edit:String = String::new();
                                        io::stdin().read_line(&mut tag_num_to_edit).expect("Failed to read line");
                                        let tag_num_to_edit_int:usize = tag_num_to_edit.trim_end().parse::<usize>().expect("Failed to parse integer of usize") - 1;
                                        println!("Index of the tag to sort by: {}\nTag to sort by: {}", tag_num_to_edit_int, tags_collection[tag_num_to_edit_int]);
                                        // ----- ^ for debugging purposes
                                        Command::new("clear").status().expect("Failed to call command");
                                        let tag_sort_criteria:&str = tags_collection[tag_num_to_edit_int];
                                        let mut sorted_task_tag_collection:Vec<&Task> = Vec::new();
                                        for i in 0..storage_vector.len() {
                                            let given_task:&Task = &storage_vector[i];
                                            let indiv_task_tag:Vec<&str> = storage_vector[i].task_tags.split("&").collect();
                                            // println!("{:?}", indiv_task_tag);
                                            for tag in indiv_task_tag {
                                                if tag == tag_sort_criteria {
                                                    sorted_task_tag_collection.push(given_task);
                                                } else {
                                                }
                                            }
                                        }
                                        // println!("{:?}", sorted_task_tag_collection);
                                        println!("{} {} {}\n", "Sorting by the".yellow(), "tag:".yellow().underline(), tag_sort_criteria.green());
                                        let mut counter:u8 = 1;
                                        for task in sorted_task_tag_collection {
                                            println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
                                            println!("{} {}", "Name: ".yellow(), task.task_name);
                                            println!("{} {}", "Description: ".yellow(), task.task_description);
                                            let mut task_deadline_string:String = String::from("");
                                            for component in task.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            task_deadline_string.pop();
                                            println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
                                            if task.task_tags == String::from(" ") {
                                                println!("{} {}\n", "Urgency: ".yellow(), task.task_urgency);
                                            } else {
                                                let task_tags_collection:Vec<&str> = task.task_tags.split("&").collect();
                                                let mut task_tags_for_reader:String = String::new();
                                                for item in task_tags_collection {
                                                    task_tags_for_reader.push_str(item);
                                                    task_tags_for_reader.push_str(", ");
                                                }
                                                task_tags_for_reader.pop();
                                                task_tags_for_reader.pop();
                                                println!("{} {}", "Urgency: ".yellow(), task.task_urgency);
                                                println!("{} {}\n", "Tags: ".yellow(), task_tags_for_reader);
                                            }
                                            counter += 1;
                                        }
                                    },

                                    _ => (),
                                    // match-all statement
                                }
                            } else {
                                println!("{}\n{}", "No tasks were found.".red().underline(), "Please create a task first".yellow());
                            }
                        },
                        
                        // match-all statement for other cases
                        &_ => {
                            Command::new("clear").status().expect("Failed to call command");
                            println!("{}\n{}", "Invalid input detected.".red().underline(), "Please give a valid input.".yellow());
                        }
                    }
            
            // -----
                     
                } else {
                    Command::new("clear").status().expect("Failed to run command");
                    // (1.2) directory is empty, handle creation of new folders
                    // println!(".kelpProjects folder exists but is empty");
                    // Command::new("clear").status().expect("Failed to run command");
                    println!("{}\n", "No active projects found.".yellow().underline());
                    // CODE THAT CREATES NEW PROJECT DIRECTORIES INSIDE THE FILE PATH
                    let mut project_name:String = String::new();
                    println!("{} {}", "Input".cyan(), "new project name:".cyan().underline());
                    io::stdin().read_line(&mut project_name).expect("Failed to read line");
                    let project_name_string:String = project_name.trim_end().to_string();
                    Command::new("clear").status().expect("Failed to run command");
                    println!("{} {}{}{}", "Starting up".yellow(), "new project ".yellow(), project_name_string.cyan().underline(), "...".yellow());
                    let mut target_file_path:String = String::from(".kelpProjects/");
                    target_file_path.push_str(project_name_string.as_str());
                    fs::create_dir(target_file_path).expect("Unable to create the desired file!");
                    thread::sleep(time::Duration::from_secs(2));
                    // PROJECT TASK LOOP = 696 lines
                    let mut file_name_list:Vec<String> = Vec::new();
                    for file in fs::read_dir(".kelpProjects").expect("Unable to read file directory") {
                        // println!("{}", file.expect("Unable to open file").path().display());
                        let file_name:String = file.expect("Unable to open file").path().display().to_string();
                        file_name_list.push(file_name);
                    }
                    Command::new("clear").status().expect("Failed to run command.");
                    // println!("{}", "File directory already exists".yellow());
                    println!("{}\n", "The following projects were found.".yellow());
                    // println!("{:?}", file_name_list);
                    let mut counter:i8 = 1;
                    for file in &file_name_list {
                        println!("{} | {}", counter, file.split("/").collect::<Vec<&str>>()[1].bright_green());
                        counter += 1;
                    }
                    let mut project_number:String = String::new();
                    println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the project you would like to work on:".yellow());
                    io::stdin().read_line(&mut project_number).expect("Failed to read line");
                    Command::new("clear").status().expect("Failed to run command");
                    let project_number_int:usize = project_number.trim_end().parse::<usize>().expect("Failed to parse number") - 1;
                    // println!("{}", project_number_int);
                    let mut target_file_path:String = file_name_list[project_number_int].to_owned();
                    target_file_path.push_str("/.kelpStorage");
                    // handle opening of folder to check whether a .kelpStorage file exists

                    // -----
                    
                    // the start of the actual task loop

                    let mut storage_vector:Vec<Task> = vec![];

                    // printing of logo 
                    Command::new("clear").status().expect("Failed to call command");

                    // println!("{}", target_file_path);

                    println!("{} {}\n", "Project name:".yellow(), target_file_path.split("/").collect::<Vec<&str>>()[1].bright_green().underline());

                    // reading of local file and parsing it into the struct Task
                    let file_contents_results = fs::read_to_string(target_file_path.clone());
                    let _file_contents = match file_contents_results {
                        Ok(string) => {
                            println!("{}\nLoading data.", "Save file found.".green().underline());
                            let file_contents_array = string.trim_end().split("\n");
                            let file_contents_vector:Vec<&str> = file_contents_array.collect();
                            for eachtask in &file_contents_vector {
                                if eachtask.chars().last().expect("Failed to find character") == ',' {
                                    let each_task_array:Vec<&str> = eachtask.split(", ").collect();
                                    let each_task_deadline_array:Vec<&str> = each_task_array[2].trim_end_matches("/").split("/").collect();
                                    let each_task_deadline:[i32;3] = [each_task_deadline_array[0].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[1].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[2].trim_end().parse().expect("Failed to parse number")];
                                    let mut each_task_urgency_unedited:String = each_task_array[3].to_string();
                                    each_task_urgency_unedited.truncate(each_task_urgency_unedited.len() -1);
                                    match each_task_urgency_unedited.parse::<UrgencyLevel>() {
                                        Ok(level) => {
                                            let each_task_urgency:UrgencyLevel = level;
                                            let the_given_task = Task {
                                                task_name: String::from(each_task_array[0]),
                                                task_description: String::from(each_task_array[1]),
                                                task_deadline: each_task_deadline,
                                                task_urgency: each_task_urgency,
                                                task_tags: String::from(" "),
                                                };
                                            storage_vector.push(the_given_task);
                                        },
                                        Err(_) => (),
                                    }
                                } else {
                                    let each_task_array:Vec<&str> = eachtask.split(", ").collect();
                                    let each_task_deadline_array:Vec<&str> = each_task_array[2].trim_end_matches("/").split("/").collect();
                                    let each_task_deadline:[i32;3] = [each_task_deadline_array[0].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[1].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[2].trim_end().parse().expect("Failed to parse number")];
                                    match each_task_array[3].parse::<UrgencyLevel>() {
                                        Ok(level) => {
                                            let each_task_urgency:UrgencyLevel = level;
                                            let the_given_task = Task {
                                                task_name: String::from(each_task_array[0]),
                                                task_description: String::from(each_task_array[1]),
                                                task_deadline: each_task_deadline,
                                                task_urgency: each_task_urgency,
                                                task_tags: String::from(each_task_array[4]),
                                                };
                                            storage_vector.push(the_given_task);
                                        },
                                        Err(_) => (),
                                    }
                                }
                            }
                            // for debugging purposes only, to be edited out in actual program
                            println!("{}\n", "Here are your tasks:".yellow());
                            display_task_vector(&storage_vector, 1);
                        },
                        Err(_) => println!("{}\n{}\n", "No save file found.".red().underline(), "Loading a fresh save.".yellow()),
                    };

                    // -----

                    // menu screen
                    println!("{}\n{}\n{}\n{}\n{}", "What would you like to do?".yellow(), "[C]reate new task".magenta(), "[E]dit a task".blue(), "[F]inish a task".cyan(), "[S]ort tasks".bright_green());
                    let mut choose_action:String = String::new();
                    io::stdin().read_line(&mut choose_action).expect("Failed to read line");
                    let choose_action_str:&str = choose_action.as_str().trim_end();

                    match choose_action_str {
                        
                        // CREATE A TASK
                        "c" => {

                            Command::new("clear").status().expect("Failed to call command");
                            // create task loop
                            loop {
                                
                                // break condition
                                println!("[E]xit / [Enter] to {}: ", "add task".bold());
                                let mut exit_condition:String = String::new();
                                io::stdin().read_line(&mut exit_condition).expect("Failed to read line");
                                let exit_condition_str:&str = exit_condition.as_str().trim_end();
                                if exit_condition_str == "e" {
                                    if storage_vector.len() > 0 {
                                        // writing of all tasks to a local file titled .kelpStorage
                                        let mut save_file = File::create(target_file_path.clone()).expect("File already exists");
                                        for eachtask in &storage_vector {
                                            let mut task_deadline_string:String = String::from("");
                                            for component in eachtask.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            let write_to_file_result = write!(save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                            match write_to_file_result {
                                                Ok(_) => (),
                                                Err(_) => (),
                                            }
                                        }
                                    } else {
                                        Command::new("clear").status().expect("Failed to call command");
                                        println!("{}\n{}", "No tasks were created.".red().underline(), "Exiting without creating save file.".yellow());
                                    }
                                    break;
                                }

                                // -----

                                // task name
                                Command::new("clear").status().expect("Failed to call command");
                                println!("{} {}{} ", "Enter".yellow(), "task name".yellow().bold(), ":".yellow());
                                let mut userinput_task_name:String = String::new();
                                io::stdin().read_line(&mut userinput_task_name).expect("Failed to read line");
                                let userinput_task_name = String::from(userinput_task_name.trim_end());

                                // -----
                                
                                // task description
                                println!("{} {}{} ", "Enter".yellow(), "task description".yellow().bold(), ":".yellow());
                                let mut userinput_task_description:String = String::new();
                                io::stdin().read_line(&mut userinput_task_description).expect("Failed to read line");
                                let userinput_task_description = String::from(userinput_task_description.trim_end());

                                // -----
                                
                                // task deadline, parsed using destructuring
                                println!("{} {} {} {}{} ", "Enter".yellow(), "task deadline".yellow().bold(), "in the following format".yellow(), "[DD/MM/YY]".underline().yellow(), ":".yellow());
                                let userinput_task_deadline_formatted:[i32; 3];

                                loop {
                                    let mut userinput_task_deadline_raw:String = String::new();
                                    io::stdin().read_line(&mut userinput_task_deadline_raw).expect("Failed to read line");
                                    let userinput_task_deadline_raw_array = userinput_task_deadline_raw.split("/");
                                    let userinput_task_deadline_array: Vec<&str> = userinput_task_deadline_raw_array.collect();
                                    
                                    // checking for valid number of fields input (characters, str literals and numbers covered)
                                    if userinput_task_deadline_array.len() != 3 {
                                        println!("{}\nEnter {} in the following format {}: ", "Invalid input detected.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                        continue;
                                    }

                                    // checking for characters instead of date input if there are 3 fields
                                    if userinput_task_deadline_array[0].chars().all(char::is_numeric) && userinput_task_deadline_array[1].chars().all(char::is_numeric) && userinput_task_deadline_array[2].trim_end().chars().all(char::is_numeric) {
                                    } else {
                                        println!("{}\nEnter {} in the following format {}: ", "Enter a valid integer input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                        continue;
                                    }

                                    // these have to be signed integers first, to allow for subsequent error checking
                                    let userinput_task_deadline_day_int:i32 = userinput_task_deadline_array[0].trim_end().parse().expect("Failed to parse number");
                                    let userinput_task_deadline_month_int:i32 = userinput_task_deadline_array[1].trim_end().parse().expect("Failed to parse number");
                                    let userinput_task_deadline_year_int:i32 = userinput_task_deadline_array[2].trim_end().parse().expect("Failed to parse number");
                                    
                                    // checking for valid date inputs
                                    if userinput_task_deadline_day_int > 31 || userinput_task_deadline_day_int < 1 {
                                        println!("{}\nEnter {} in the following format {}: ", "Enter a valid day input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                        continue;
                                    }
                                    if userinput_task_deadline_month_int > 12 || userinput_task_deadline_month_int < 1 {
                                        println!("{}\nEnter {} in the following format {}: ", "Enter a valid month input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                        continue;
                                    } 
                                    if userinput_task_deadline_year_int < 23 || userinput_task_deadline_year_int > 99 {
                                        println!("{}\nEnter {} in the following format {}: ", "Enter a valid year input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                        continue; 
                                    }
                                    userinput_task_deadline_formatted = [userinput_task_deadline_day_int, userinput_task_deadline_month_int, userinput_task_deadline_year_int];
                                    break;
                                }
                                
                                // -----

                                // task urgency, handled by an enum
                                println!("{} {} {}{} ", "Enter".yellow(), "task urgency".yellow().bold(), "[L/M/H]".yellow().underline(), ":".yellow());
                                let userinput_task_urgency:UrgencyLevel;
                                
                                loop {
                                    let mut userinput_task_urgency_string:String = String::new();
                                    io::stdin().read_line(&mut userinput_task_urgency_string).expect("Failed to read line");
                                    let userinput_task_urgency_stringliteral:&str = userinput_task_urgency_string.as_str().trim_end();
                                    match userinput_task_urgency_stringliteral {
                                        "l" => {
                                            userinput_task_urgency = UrgencyLevel::Low;
                                            break;
                                        },
                                        "m" => {
                                            userinput_task_urgency = UrgencyLevel::Medium;
                                            break;
                                        },
                                        "h" => {
                                            userinput_task_urgency = UrgencyLevel::High;
                                            break;
                                        },
                                        // match-all pattern employed for invalid input
                                        &_ => {
                                            println!("{} [L/M/H]: ", "Please enter a valid input!".red().underline());
                                            }
                                        }
                                    }
                                
                                // -----
                                
                                // task tags added to a collection that can then be iterated over
                                println!("{} {}{} {}{}", "Enter".yellow(), "task tags".yellow(), ",".yellow(), "separated by a space".yellow().underline(), ":".yellow());

                                let mut userinput_task_tag:String = String::new();
                                io::stdin().read_line(&mut userinput_task_tag).expect("Failed to read line.");
                                let userinput_task_tag_collection:Vec<&str> = userinput_task_tag.trim_end().split(" ").collect();
                                let userinput_task_tag_collection_formatted:String = userinput_task_tag_collection.join("&");

                                // -----
                                
                                // creation of an instance of the Task struct, and assignment of internal field values
                                let given_task = Task {
                                    task_name: userinput_task_name,
                                    task_description: userinput_task_description,
                                    task_deadline: userinput_task_deadline_formatted,
                                    task_urgency: userinput_task_urgency,
                                    task_tags: userinput_task_tag_collection_formatted,
                                };
                                
                                // updating of storage_vector:Vec<Task> collection
                                storage_vector.push(given_task);
                                Command::new("clear").status().expect("Failed to call command");

                                };
                                
                            if storage_vector.len() > 0 {
                                Command::new("clear").status().expect("Failed to call command");
                                println!("{}\n", "Here are your tasks: ".yellow());
                                let mut counter:i32 = 1;
                                for task in storage_vector {
                                    if task.task_tags == "" || task.task_tags == " " {
                                        println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
                                        println!("{} {}", "Name: ".yellow(), task.task_name);
                                        println!("{} {}", "Description: ".yellow(), task.task_description);
                                        let mut task_deadline_string:String = String::from("");
                                        for component in task.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        task_deadline_string.pop();
                                        println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
                                        println!("{} {}\n", "Urgency: ".yellow(), task.task_urgency);
                                    } else {
                                        println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
                                        println!("{} {}", "Name: ".yellow(), task.task_name);
                                        println!("{} {}", "Description: ".yellow(), task.task_description);
                                        let mut task_deadline_string:String = String::from("");
                                        for component in task.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        task_deadline_string.pop();
                                        println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
                                        println!("{} {}", "Urgency: ".yellow(), task.task_urgency);
                                        let task_tags_collection:Vec<&str> = task.task_tags.split("&").collect();
                                        let mut task_tags_for_reader:String = String::new();
                                        for item in task_tags_collection {
                                            task_tags_for_reader.push_str(item);
                                            task_tags_for_reader.push_str(", ");
                                        }
                                        task_tags_for_reader.pop();
                                        task_tags_for_reader.pop();
                                        println!("{} {}\n", "Tags: ".yellow(), task_tags_for_reader);
                                    }
                                        counter += 1;
                                }
                            }
                        }, 
                        
                        // EDIT A TASK
                        "e" => {
                            Command::new("clear").status().expect("Failed to call command");
                            // .unwrap() is used for error handling here
                            if storage_vector.len() > 0 {
                                println!("{}\n", "Here are your tasks: ".yellow());
                                let mut counter:u8 = 1;
                                for task in &storage_vector {
                                    println!("{}. | {:?} ", counter, task.task_name);
                                    counter += 1;
                                }
                                println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the task you would like to edit:".yellow());
                                let mut task_to_edit:String = String::new();
                                io::stdin().read_line(&mut task_to_edit).expect("Failed to read line");
                                let task_to_edit_int:usize = task_to_edit.trim_end().parse::<usize>().expect("Failed to parse integer of usize") - 1;
                                // println!("{}", task_to_edit_int);
                                // println!("Index of the task to be edited: {}", task_to_edit_int);
                                // println!("{:?}", storage_vector[task_to_edit_int].task_name);               
                                // ----- ^ for debugging purposes
                                Command::new("clear").status().expect("Failed to call command");
                                println!("{}\n{}\n{}\n{}\n{}\n{}", "Which component of the task do you want to edit?".yellow(), "[N]ame".purple(), "[D]escription".blue(), "D[E]adline".cyan(), "[U]rgency".bright_green(), "[T]ags".bright_red());
                                let mut what_to_edit:String = String::new();
                                io::stdin().read_line(&mut what_to_edit).expect("Failed to read line");
                                // could use .unwrap() for error handling above as well
                                let what_to_edit_str = what_to_edit.as_str().trim_end();
                                match what_to_edit_str {
                                    "n" => {
                                        let storage_vector = edit_task_name(task_to_edit_int, storage_vector);
                                        let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                        for eachtask in storage_vector {
                                            let mut task_deadline_string:String = String::from("");
                                            for component in eachtask.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                            match write_to_file_result {
                                                Ok(_) => (),
                                                Err(_) => (),
                                            }
                                        }
                                    },
                                    "d" => {
                                        let storage_vector = edit_task_description(task_to_edit_int, storage_vector);
                                        let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                        for eachtask in storage_vector {
                                            let mut task_deadline_string:String = String::from("");
                                            for component in eachtask.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                            match write_to_file_result {
                                                Ok(_) => (),
                                                Err(_) => (),
                                            }
                                        }
                                    },
                                    "e" => {
                                        let storage_vector = edit_task_deadline(task_to_edit_int, storage_vector);
                                        let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                        for eachtask in storage_vector {
                                            let mut task_deadline_string:String = String::from("");
                                            for component in eachtask.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                            match write_to_file_result {
                                                Ok(_) => (),
                                                Err(_) => (),
                                            }
                                        }
                                    },
                                    "u" => {
                                        let storage_vector = edit_task_urgency(task_to_edit_int, storage_vector);
                                        let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                        for eachtask in storage_vector {
                                            let mut task_deadline_string:String = String::from("");
                                            for component in eachtask.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                            match write_to_file_result {
                                                Ok(_) => (),
                                                Err(_) => (),
                                            }
                                        }
                                    },
                                    "t" => {
                                        let storage_vector = edit_task_tags(task_to_edit_int, storage_vector);
                                        let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                        for eachtask in storage_vector {
                                            let mut task_deadline_string:String = String::from("");
                                            for component in eachtask.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                            match write_to_file_result {
                                                Ok(_) => (),
                                                Err(_) => (),
                                            }
                                        }
                                    }
                                    _ => (),
                                    // match-all statement
                                };
                            } else {
                                println!("{}\n{}", "No tasks were found.".red().underline(), "Please create a task first".yellow());
                            }
                        }, 

                        // FINISH A TASK
                        "f" => {
                            Command::new("clear").status().expect("Failed to call command");
                            // .unwrap() is used for error handling here
                            if storage_vector.len() > 0 {
                                println!("{}\n", "Here are your tasks: ".yellow());
                                let mut counter:u8 = 1;
                                for task in &storage_vector {
                                    println!("{}. | {:?} ", counter, task.task_name);
                                    counter += 1;
                                }
                                println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the task you have completed:".yellow());
                                let mut completed_task:String = String::new();
                                io::stdin().read_line(&mut completed_task).expect("Failed to read line");
                                let completed_task_int:usize = completed_task.trim_end().parse::<usize>().expect("Failed to parse integer of usize") - 1;
                                // println!("{}", completed_task_int);
                                Command::new("clear").status().expect("Failed to call command");
                                let removed_task = storage_vector.remove(completed_task_int);
                                println!("{} {}", removed_task.task_name.yellow().underline(), "has been completed!".yellow());
                                println!("{}\n", "Good job! Remember to take breaks and drink enough water!".green());
                                // println!("{:?}", storage_vector);
                                if storage_vector.is_empty() {
                                    println!("{}\n{}", "No outstanding tasks left!".yellow().underline(), "Go for a run :)".green().bold());
                                    fs::remove_file(target_file_path.clone()).expect("Failed to find file");
                                    // simply removes the local file to prevent an empty vector from being saved
                                } else {
                                    let storage_vector_len:usize = storage_vector.len();
                                    if storage_vector_len == 1 {
                                        println!("{} {}\n", "You have".yellow(), "1 outstanding task.".yellow().underline());
                                    } else {
                                        let storage_vector_len_string:String = storage_vector_len.to_string();
                                        println!("{} {}{}\n", "You have".yellow(), storage_vector_len_string.yellow().underline(), " outstanding tasks.".yellow().underline());
                                    }

                                    let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                    for eachtask in storage_vector {
                                        let mut task_deadline_string:String = String::from("");
                                        for component in eachtask.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                        match write_to_file_result {
                                            Ok(_) => (),
                                            Err(_) => (),
                                        }
                                    }
                                    // creates and rewrites the local file with the updated storage_vector
                                }
                            } else {
                                println!("{}\n{}", "No tasks were found.".red().underline(), "Please create a task first".yellow());
                            }
                        },
                        
                        // SORT TASKS
                        "s" => {
                            Command::new("clear").status().expect("Failed to call command");
                            if storage_vector.len() > 0 {
                                println!("{}\n", "Here are your tasks: ".yellow());
                                let mut counter:u8 = 1;
                                for task in &storage_vector {
                                    println!("{}. | {:?} ", counter, task.task_name);
                                    counter += 1;
                                }
                                println!("\n{}\n{}\n{}\n{}", "Sort tasks by...".yellow(), "[U]rgency".purple(), "D[E]adline".cyan(), "[T]ags".green());
                                let mut sort_criteria:String = String::new();
                                io::stdin().read_line(&mut sort_criteria).expect("Failed to read line");
                                let sort_criteria_str:&str = sort_criteria.as_str().trim_end();
                                match sort_criteria_str {
                                    "u" => {
                                        let mut low_urgency_storage_vector:Vec<Task> = vec![];
                                        let mut medium_urgency_storage_vector:Vec<Task> = vec![];
                                        let mut high_urgency_storage_vector:Vec<Task> = vec![];
                                        Command::new("clear").status().expect("Failed to call command");
                                        println!("{} {}", "Sorting by".yellow(), "urgency level.".yellow().underline().bold());
                                        for task in storage_vector {
                                            match task.task_urgency {
                                                UrgencyLevel::Low => {
                                                    low_urgency_storage_vector.push(task);
                                                },
                                                UrgencyLevel::Medium => {
                                                    medium_urgency_storage_vector.push(task);
                                                },
                                                UrgencyLevel::High => {
                                                    high_urgency_storage_vector.push(task);
                                                }, 
                                                // note that there is no need for a match-all statement since an
                                                // enum neccesitates that only its enum variants can fulfill its
                                                // requirements
                                            }
                                        }

                                        counter = 1;

                                        println!("\n{}\n", "High urgency tasks".red());
                                        if high_urgency_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&high_urgency_storage_vector, counter);
                                        };
                                        
                                        println!("\n{}\n", "Medium urgency tasks".blue());
                                        if medium_urgency_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&medium_urgency_storage_vector, counter);
                                        };

                                        println!("\n{}\n", "Low urgency tasks".green());
                                        if low_urgency_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&low_urgency_storage_vector, counter);
                                        };

                                    },

                                    "e" => {
                                        Command::new("clear").status().expect("Failed to call command");
                                        let mut overdue_storage_vector:Vec<Task> = vec![];
                                        let mut today_storage_vector:Vec<Task> = vec![];
                                        let mut this_month_storage_vector:Vec<Task> = vec![];
                                        let mut next_month_and_later_storage_vector:Vec<Task> = vec![];
                                        let current_day:String = Local::now().to_string();
                                        let current_day_str:&str = current_day.as_str();
                                        let current_day_vector:Vec<&str> = current_day_str.split(" ").collect();
                                        let current_date_vector:Vec<&str> = current_day_vector[0].split("-").collect();
                                        // println!("{:?}", current_date_vector);
                                        let current_year:i32 = current_date_vector[0][2..].parse().expect("Failed to parse number");
                                        let current_month:i32 = current_date_vector[1].parse().expect("Failed to parse number");
                                        let current_day_of_date:i32 = current_date_vector[2].parse().expect("Failed to parse number");
                                        println!("{} {}", "Sorting by".yellow(), "deadline.".yellow().underline());
                                        for task in storage_vector {
                                            if task.task_deadline[2] > current_year {
                                                // deadline is a year or more later
                                                next_month_and_later_storage_vector.push(task);
                                            } else if task.task_deadline[1] > current_month {
                                                // deadline is a month or more later
                                                next_month_and_later_storage_vector.push(task);
                                            } else if task.task_deadline[0] > current_day_of_date {
                                                // deadline is a day or more later
                                                this_month_storage_vector.push(task);
                                            } else if task.task_deadline[0] == current_day_of_date {
                                                // deadline is today
                                                today_storage_vector.push(task);
                                            } else {
                                                // deadline is overdue 
                                                overdue_storage_vector.push(task);
                                            }
                                        };

                                        let counter:u8 = 1;

                                        println!("\n{}\n", "Overdue tasks".red());
                                        if overdue_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&overdue_storage_vector, counter);
                                        };

                                        println!("\n{}\n", "Tasks due today".red());
                                        if today_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&today_storage_vector, counter);
                                        };

                                        println!("\n{}\n", "Tasks due this month".blue());
                                        if this_month_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&this_month_storage_vector, counter);
                                        };

                                        println!("\n{}\n", "Tasks due next month".green());
                                        if next_month_and_later_storage_vector.is_empty() {
                                            println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                        } else {
                                            display_task_vector(&next_month_and_later_storage_vector, counter);
                                        };

                                    }, 

                                    "t" => {
                                        Command::new("clear").status().expect("Failed to call command");
                                        println!("{} {}\n", "Sorting by".yellow(), "tag type.".yellow().underline());
                                        println!("{}\n", "Here are your tags:".yellow());
                                        let mut tags_collection:Vec<&str> = Vec::new();
                                        for task in &storage_vector {
                                            let indiv_task_tag:Vec<&str> = task.task_tags.split("&").collect();
                                            // println!("{:?}", indiv_task_tag);
                                            for tag in indiv_task_tag {
                                                if tag == " " {
                                                } else {
                                                    tags_collection.push(tag);
                                                }
                                            }
                                        }
                                        tags_collection.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                                        // ^ sort a vector
                                        // println!("{:?}", tags_collection);
                                        tags_collection.dedup(); 
                                        // ^ removes duplicates
                                        // println!("{:?}", tags_collection);

                                        if tags_collection.len() > 0 {
                                            let mut counter:u8 = 1;
                                            for tag in &tags_collection {
                                                println!("{}. | {:?} ", counter, tag);
                                                counter += 1;
                                                }
                                            }
                                        println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the tag you would like to sort by:".yellow());
                                        let mut tag_num_to_edit:String = String::new();
                                        io::stdin().read_line(&mut tag_num_to_edit).expect("Failed to read line");
                                        let tag_num_to_edit_int:usize = tag_num_to_edit.trim_end().parse::<usize>().expect("Failed to parse integer of usize") - 1;
                                        println!("Index of the tag to sort by: {}\nTag to sort by: {}", tag_num_to_edit_int, tags_collection[tag_num_to_edit_int]);
                                        // ----- ^ for debugging purposes
                                        Command::new("clear").status().expect("Failed to call command");
                                        let tag_sort_criteria:&str = tags_collection[tag_num_to_edit_int];
                                        let mut sorted_task_tag_collection:Vec<&Task> = Vec::new();
                                        for i in 0..storage_vector.len() {
                                            let given_task:&Task = &storage_vector[i];
                                            let indiv_task_tag:Vec<&str> = storage_vector[i].task_tags.split("&").collect();
                                            // println!("{:?}", indiv_task_tag);
                                            for tag in indiv_task_tag {
                                                if tag == tag_sort_criteria {
                                                    sorted_task_tag_collection.push(given_task);
                                                } else {
                                                }
                                            }
                                        }
                                        // println!("{:?}", sorted_task_tag_collection);
                                        println!("{} {} {}\n", "Sorting by the".yellow(), "tag:".yellow().underline(), tag_sort_criteria.green());
                                        let mut counter:u8 = 1;
                                        for task in sorted_task_tag_collection {
                                            println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
                                            println!("{} {}", "Name: ".yellow(), task.task_name);
                                            println!("{} {}", "Description: ".yellow(), task.task_description);
                                            let mut task_deadline_string:String = String::from("");
                                            for component in task.task_deadline {
                                                task_deadline_string.push_str(component.to_string().as_str());
                                                task_deadline_string.push_str("/");
                                            };
                                            task_deadline_string.pop();
                                            println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
                                            if task.task_tags == String::from(" ") {
                                                println!("{} {}\n", "Urgency: ".yellow(), task.task_urgency);
                                            } else {
                                                let task_tags_collection:Vec<&str> = task.task_tags.split("&").collect();
                                                let mut task_tags_for_reader:String = String::new();
                                                for item in task_tags_collection {
                                                    task_tags_for_reader.push_str(item);
                                                    task_tags_for_reader.push_str(", ");
                                                }
                                                task_tags_for_reader.pop();
                                                task_tags_for_reader.pop();
                                                println!("{} {}", "Urgency: ".yellow(), task.task_urgency);
                                                println!("{} {}\n", "Tags: ".yellow(), task_tags_for_reader);
                                            }
                                            counter += 1;
                                        }
                                    },

                                    _ => (),
                                    // match-all statement
                                }
                            } else {
                                println!("{}\n{}", "No tasks were found.".red().underline(), "Please create a task first".yellow());
                            }
                        },
                        
                        // match-all statement for other cases
                        &_ => {
                            Command::new("clear").status().expect("Failed to call command");
                            println!("{}\n{}", "Invalid input detected.".red().underline(), "Please give a valid input.".yellow());
                        }
                    }
                }
            } else {
                Command::new("clear").status().expect("Failed to run command");
                // (2) file directory does not exist --> first time users
                // handle subsequent creation of folders and tasks
                println!("{}", "No projects folder found!".red().underline());
                fs::create_dir(".kelpProjects").expect("Unable to create the desired file!");
                println!("{}", "Projects folder created.".green());
                // CODE THAT CREATES NEW PROJECT DIRECTORIES INSIDE THE FILE PATH
                let mut project_name:String = String::new();
                println!("\n{} {}", "Input".cyan(), "new project name:".cyan().underline());
                io::stdin().read_line(&mut project_name).expect("Failed to read line");
                let project_name_string:String = project_name.trim_end().to_string();
                Command::new("clear").status().expect("Failed to run command");
                println!("{} {}{}{}", "Starting up".yellow(), "new project ".yellow(), project_name_string.cyan().underline(), "...".yellow());
                let mut target_file_path:String = String::from(".kelpProjects/");
                target_file_path.push_str(project_name_string.as_str());
                fs::create_dir(target_file_path).expect("Unable to create the desired file!");
                thread::sleep(time::Duration::from_secs(2));
                let mut file_name_list:Vec<String> = Vec::new();
                for file in fs::read_dir(".kelpProjects").expect("Unable to read file directory") {
                    let file_name:String = file.expect("Unable to open file").path().display().to_string();
                    file_name_list.push(file_name);
                }
                Command::new("clear").status().expect("Failed to run command.");
                // println!("{}", "File directory already exists".yellow());
                println!("{}\n", "Here are your projects.".yellow());
                // println!("{:?}", file_name_list);
                let mut counter:i8 = 1;
                for file in &file_name_list {
                    println!("{} | {}", counter, file.split("/").collect::<Vec<&str>>()[1].bright_green());
                    counter += 1;
                }
                let mut project_number:String = String::new();
                println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the project you would like to work on:".yellow());
                io::stdin().read_line(&mut project_number).expect("Failed to read line");
                Command::new("clear").status().expect("Failed to run command");
                let project_number_int:usize = project_number.trim_end().parse::<usize>().expect("Failed to parse number") - 1;
                // println!("{}", project_number_int);
                let mut target_file_path:String = file_name_list[project_number_int].to_owned();
                target_file_path.push_str("/.kelpStorage");
                // handle opening of folder to check whether a .kelpStorage file exists

                // -----
                
                // the start of the actual task loop

                let mut storage_vector:Vec<Task> = vec![];

                // printing of logo 
                Command::new("clear").status().expect("Failed to call command");

                // println!("{}", target_file_path);

                println!("{} {}\n", "Project name:".yellow(), target_file_path.split("/").collect::<Vec<&str>>()[1].bright_green().underline());

                // reading of local file and parsing it into the struct Task
                let file_contents_results = fs::read_to_string(target_file_path.clone());
                let _file_contents = match file_contents_results {
                    Ok(string) => {
                        println!("{}\nLoading data.", "Save file found.".green().underline());
                        let file_contents_array = string.trim_end().split("\n");
                        let file_contents_vector:Vec<&str> = file_contents_array.collect();
                        for eachtask in &file_contents_vector {
                            if eachtask.chars().last().expect("Failed to find character") == ',' {
                                let each_task_array:Vec<&str> = eachtask.split(", ").collect();
                                let each_task_deadline_array:Vec<&str> = each_task_array[2].trim_end_matches("/").split("/").collect();
                                let each_task_deadline:[i32;3] = [each_task_deadline_array[0].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[1].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[2].trim_end().parse().expect("Failed to parse number")];
                                let mut each_task_urgency_unedited:String = each_task_array[3].to_string();
                                each_task_urgency_unedited.truncate(each_task_urgency_unedited.len() -1);
                                match each_task_urgency_unedited.parse::<UrgencyLevel>() {
                                    Ok(level) => {
                                        let each_task_urgency:UrgencyLevel = level;
                                        let the_given_task = Task {
                                            task_name: String::from(each_task_array[0]),
                                            task_description: String::from(each_task_array[1]),
                                            task_deadline: each_task_deadline,
                                            task_urgency: each_task_urgency,
                                            task_tags: String::from(" "),
                                            };
                                        storage_vector.push(the_given_task);
                                    },
                                    Err(_) => (),
                                }
                            } else {
                                let each_task_array:Vec<&str> = eachtask.split(", ").collect();
                                let each_task_deadline_array:Vec<&str> = each_task_array[2].trim_end_matches("/").split("/").collect();
                                let each_task_deadline:[i32;3] = [each_task_deadline_array[0].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[1].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[2].trim_end().parse().expect("Failed to parse number")];
                                match each_task_array[3].parse::<UrgencyLevel>() {
                                    Ok(level) => {
                                        let each_task_urgency:UrgencyLevel = level;
                                        let the_given_task = Task {
                                            task_name: String::from(each_task_array[0]),
                                            task_description: String::from(each_task_array[1]),
                                            task_deadline: each_task_deadline,
                                            task_urgency: each_task_urgency,
                                            task_tags: String::from(each_task_array[4]),
                                            };
                                        storage_vector.push(the_given_task);
                                    },
                                    Err(_) => (),
                                }
                            }
                        }
                        // for debugging purposes only, to be edited out in actual program
                        println!("{}\n", "Here are your tasks:".yellow());
                        display_task_vector(&storage_vector, 1);
                    },
                    Err(_) => println!("{}\n{}\n", "No save file found.".red().underline(), "Loading a fresh save.".yellow()),
                };

                // -----

                // menu screen
                println!("{}\n{}\n{}\n{}\n{}", "What would you like to do?".yellow(), "[C]reate new task".magenta(), "[E]dit a task".blue(), "[F]inish a task".cyan(), "[S]ort tasks".bright_green());
                let mut choose_action:String = String::new();
                io::stdin().read_line(&mut choose_action).expect("Failed to read line");
                let choose_action_str:&str = choose_action.as_str().trim_end();

                match choose_action_str {
                    
                    // CREATE A TASK
                    "c" => {

                        Command::new("clear").status().expect("Failed to call command");
                        // create task loop
                        loop {
                            
                            // break condition
                            println!("[E]xit / [Enter] to {}: ", "add task".bold());
                            let mut exit_condition:String = String::new();
                            io::stdin().read_line(&mut exit_condition).expect("Failed to read line");
                            let exit_condition_str:&str = exit_condition.as_str().trim_end();
                            if exit_condition_str == "e" {
                                if storage_vector.len() > 0 {
                                    // writing of all tasks to a local file titled .kelpStorage
                                    let mut save_file = File::create(target_file_path.clone()).expect("File already exists");
                                    for eachtask in &storage_vector {
                                        let mut task_deadline_string:String = String::from("");
                                        for component in eachtask.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        let write_to_file_result = write!(save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                        match write_to_file_result {
                                            Ok(_) => (),
                                            Err(_) => (),
                                        }
                                    }
                                } else {
                                    Command::new("clear").status().expect("Failed to call command");
                                    println!("{}\n{}", "No tasks were created.".red().underline(), "Exiting without creating save file.".yellow());
                                }
                                break;
                            }

                            // -----

                            // task name
                            Command::new("clear").status().expect("Failed to call command");
                            println!("{} {}{} ", "Enter".yellow(), "task name".yellow().bold(), ":".yellow());
                            let mut userinput_task_name:String = String::new();
                            io::stdin().read_line(&mut userinput_task_name).expect("Failed to read line");
                            let userinput_task_name = String::from(userinput_task_name.trim_end());

                            // -----
                            
                            // task description
                            println!("{} {}{} ", "Enter".yellow(), "task description".yellow().bold(), ":".yellow());
                            let mut userinput_task_description:String = String::new();
                            io::stdin().read_line(&mut userinput_task_description).expect("Failed to read line");
                            let userinput_task_description = String::from(userinput_task_description.trim_end());

                            // -----
                            
                            // task deadline, parsed using destructuring
                            println!("{} {} {} {}{} ", "Enter".yellow(), "task deadline".yellow().bold(), "in the following format".yellow(), "[DD/MM/YY]".underline().yellow(), ":".yellow());
                            let userinput_task_deadline_formatted:[i32; 3];

                            loop {
                                let mut userinput_task_deadline_raw:String = String::new();
                                io::stdin().read_line(&mut userinput_task_deadline_raw).expect("Failed to read line");
                                let userinput_task_deadline_raw_array = userinput_task_deadline_raw.split("/");
                                let userinput_task_deadline_array: Vec<&str> = userinput_task_deadline_raw_array.collect();
                                
                                // checking for valid number of fields input (characters, str literals and numbers covered)
                                if userinput_task_deadline_array.len() != 3 {
                                    println!("{}\nEnter {} in the following format {}: ", "Invalid input detected.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                    continue;
                                }

                                // checking for characters instead of date input if there are 3 fields
                                if userinput_task_deadline_array[0].chars().all(char::is_numeric) && userinput_task_deadline_array[1].chars().all(char::is_numeric) && userinput_task_deadline_array[2].trim_end().chars().all(char::is_numeric) {
                                } else {
                                    println!("{}\nEnter {} in the following format {}: ", "Enter a valid integer input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                    continue;
                                }

                                // these have to be signed integers first, to allow for subsequent error checking
                                let userinput_task_deadline_day_int:i32 = userinput_task_deadline_array[0].trim_end().parse().expect("Failed to parse number");
                                let userinput_task_deadline_month_int:i32 = userinput_task_deadline_array[1].trim_end().parse().expect("Failed to parse number");
                                let userinput_task_deadline_year_int:i32 = userinput_task_deadline_array[2].trim_end().parse().expect("Failed to parse number");
                                
                                // checking for valid date inputs
                                if userinput_task_deadline_day_int > 31 || userinput_task_deadline_day_int < 1 {
                                    println!("{}\nEnter {} in the following format {}: ", "Enter a valid day input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                    continue;
                                }
                                if userinput_task_deadline_month_int > 12 || userinput_task_deadline_month_int < 1 {
                                    println!("{}\nEnter {} in the following format {}: ", "Enter a valid month input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                    continue;
                                } 
                                if userinput_task_deadline_year_int < 23 || userinput_task_deadline_year_int > 99 {
                                    println!("{}\nEnter {} in the following format {}: ", "Enter a valid year input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                    continue; 
                                }
                                userinput_task_deadline_formatted = [userinput_task_deadline_day_int, userinput_task_deadline_month_int, userinput_task_deadline_year_int];
                                break;
                            }
                            
                            // -----

                            // task urgency, handled by an enum
                            println!("{} {} {}{} ", "Enter".yellow(), "task urgency".yellow().bold(), "[L/M/H]".yellow().underline(), ":".yellow());
                            let userinput_task_urgency:UrgencyLevel;
                            
                            loop {
                                let mut userinput_task_urgency_string:String = String::new();
                                io::stdin().read_line(&mut userinput_task_urgency_string).expect("Failed to read line");
                                let userinput_task_urgency_stringliteral:&str = userinput_task_urgency_string.as_str().trim_end();
                                match userinput_task_urgency_stringliteral {
                                    "l" => {
                                        userinput_task_urgency = UrgencyLevel::Low;
                                        break;
                                    },
                                    "m" => {
                                        userinput_task_urgency = UrgencyLevel::Medium;
                                        break;
                                    },
                                    "h" => {
                                        userinput_task_urgency = UrgencyLevel::High;
                                        break;
                                    },
                                    // match-all pattern employed for invalid input
                                    &_ => {
                                        println!("{} [L/M/H]: ", "Please enter a valid input!".red().underline());
                                        }
                                    }
                                }
                            
                            // -----
                            
                            // task tags added to a collection that can then be iterated over
                            println!("{} {}{} {}{}", "Enter".yellow(), "task tags".yellow(), ",".yellow(), "separated by a space".yellow().underline(), ":".yellow());

                            let mut userinput_task_tag:String = String::new();
                            io::stdin().read_line(&mut userinput_task_tag).expect("Failed to read line.");
                            let userinput_task_tag_collection:Vec<&str> = userinput_task_tag.trim_end().split(" ").collect();
                            let userinput_task_tag_collection_formatted:String = userinput_task_tag_collection.join("&");

                            // -----
                            
                            // creation of an instance of the Task struct, and assignment of internal field values
                            let given_task = Task {
                                task_name: userinput_task_name,
                                task_description: userinput_task_description,
                                task_deadline: userinput_task_deadline_formatted,
                                task_urgency: userinput_task_urgency,
                                task_tags: userinput_task_tag_collection_formatted,
                            };
                            
                            // updating of storage_vector:Vec<Task> collection
                            storage_vector.push(given_task);
                            Command::new("clear").status().expect("Failed to call command");

                            };
                            
                        if storage_vector.len() > 0 {
                            Command::new("clear").status().expect("Failed to call command");
                            println!("{}\n", "Here are your tasks: ".yellow());
                            let mut counter:i32 = 1;
                            for task in storage_vector {
                                if task.task_tags == "" || task.task_tags == " " {
                                    println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
                                    println!("{} {}", "Name: ".yellow(), task.task_name);
                                    println!("{} {}", "Description: ".yellow(), task.task_description);
                                    let mut task_deadline_string:String = String::from("");
                                    for component in task.task_deadline {
                                        task_deadline_string.push_str(component.to_string().as_str());
                                        task_deadline_string.push_str("/");
                                    };
                                    task_deadline_string.pop();
                                    println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
                                    println!("{} {}\n", "Urgency: ".yellow(), task.task_urgency);
                                } else {
                                    println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
                                    println!("{} {}", "Name: ".yellow(), task.task_name);
                                    println!("{} {}", "Description: ".yellow(), task.task_description);
                                    let mut task_deadline_string:String = String::from("");
                                    for component in task.task_deadline {
                                        task_deadline_string.push_str(component.to_string().as_str());
                                        task_deadline_string.push_str("/");
                                    };
                                    task_deadline_string.pop();
                                    println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
                                    println!("{} {}", "Urgency: ".yellow(), task.task_urgency);
                                    let task_tags_collection:Vec<&str> = task.task_tags.split("&").collect();
                                    let mut task_tags_for_reader:String = String::new();
                                    for item in task_tags_collection {
                                        task_tags_for_reader.push_str(item);
                                        task_tags_for_reader.push_str(", ");
                                    }
                                    task_tags_for_reader.pop();
                                    task_tags_for_reader.pop();
                                    println!("{} {}\n", "Tags: ".yellow(), task_tags_for_reader);
                                }
                                    counter += 1;
                            }
                        }
                    }, 
                    
                    // EDIT A TASK
                    "e" => {
                        Command::new("clear").status().expect("Failed to call command");
                        // .unwrap() is used for error handling here
                        if storage_vector.len() > 0 {
                            println!("{}\n", "Here are your tasks: ".yellow());
                            let mut counter:u8 = 1;
                            for task in &storage_vector {
                                println!("{}. | {:?} ", counter, task.task_name);
                                counter += 1;
                            }
                            println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the task you would like to edit:".yellow());
                            let mut task_to_edit:String = String::new();
                            io::stdin().read_line(&mut task_to_edit).expect("Failed to read line");
                            let task_to_edit_int:usize = task_to_edit.trim_end().parse::<usize>().expect("Failed to parse integer of usize") - 1;
                            // println!("{}", task_to_edit_int);
                            // println!("Index of the task to be edited: {}", task_to_edit_int);
                            // println!("{:?}", storage_vector[task_to_edit_int].task_name);               
                            // ----- ^ for debugging purposes
                            Command::new("clear").status().expect("Failed to call command");
                            println!("{}\n{}\n{}\n{}\n{}\n{}", "Which component of the task do you want to edit?".yellow(), "[N]ame".purple(), "[D]escription".blue(), "D[E]adline".cyan(), "[U]rgency".bright_green(), "[T]ags".bright_red());
                            let mut what_to_edit:String = String::new();
                            io::stdin().read_line(&mut what_to_edit).expect("Failed to read line");
                            // could use .unwrap() for error handling above as well
                            let what_to_edit_str = what_to_edit.as_str().trim_end();
                            match what_to_edit_str {
                                "n" => {
                                    let storage_vector = edit_task_name(task_to_edit_int, storage_vector);
                                    let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                    for eachtask in storage_vector {
                                        let mut task_deadline_string:String = String::from("");
                                        for component in eachtask.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                        match write_to_file_result {
                                            Ok(_) => (),
                                            Err(_) => (),
                                        }
                                    }
                                },
                                "d" => {
                                    let storage_vector = edit_task_description(task_to_edit_int, storage_vector);
                                    let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                    for eachtask in storage_vector {
                                        let mut task_deadline_string:String = String::from("");
                                        for component in eachtask.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                        match write_to_file_result {
                                            Ok(_) => (),
                                            Err(_) => (),
                                        }
                                    }
                                },
                                "e" => {
                                    let storage_vector = edit_task_deadline(task_to_edit_int, storage_vector);
                                    let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                    for eachtask in storage_vector {
                                        let mut task_deadline_string:String = String::from("");
                                        for component in eachtask.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                        match write_to_file_result {
                                            Ok(_) => (),
                                            Err(_) => (),
                                        }
                                    }
                                },
                                "u" => {
                                    let storage_vector = edit_task_urgency(task_to_edit_int, storage_vector);
                                    let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                    for eachtask in storage_vector {
                                        let mut task_deadline_string:String = String::from("");
                                        for component in eachtask.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                        match write_to_file_result {
                                            Ok(_) => (),
                                            Err(_) => (),
                                        }
                                    }
                                },
                                "t" => {
                                    let storage_vector = edit_task_tags(task_to_edit_int, storage_vector);
                                    let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                    for eachtask in storage_vector {
                                        let mut task_deadline_string:String = String::from("");
                                        for component in eachtask.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                        match write_to_file_result {
                                            Ok(_) => (),
                                            Err(_) => (),
                                        }
                                    }
                                }
                                _ => (),
                                // match-all statement
                            };
                        } else {
                            println!("{}\n{}", "No tasks were found.".red().underline(), "Please create a task first".yellow());
                        }
                    }, 

                    // FINISH A TASK
                    "f" => {
                        Command::new("clear").status().expect("Failed to call command");
                        // .unwrap() is used for error handling here
                        if storage_vector.len() > 0 {
                            println!("{}\n", "Here are your tasks: ".yellow());
                            let mut counter:u8 = 1;
                            for task in &storage_vector {
                                println!("{}. | {:?} ", counter, task.task_name);
                                counter += 1;
                            }
                            println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the task you have completed:".yellow());
                            let mut completed_task:String = String::new();
                            io::stdin().read_line(&mut completed_task).expect("Failed to read line");
                            let completed_task_int:usize = completed_task.trim_end().parse::<usize>().expect("Failed to parse integer of usize") - 1;
                            // println!("{}", completed_task_int);
                            Command::new("clear").status().expect("Failed to call command");
                            let removed_task = storage_vector.remove(completed_task_int);
                            println!("{} {}", removed_task.task_name.yellow().underline(), "has been completed!".yellow());
                            println!("{}\n", "Good job! Remember to take breaks and drink enough water!".green());
                            // println!("{:?}", storage_vector);
                            if storage_vector.is_empty() {
                                println!("{}\n{}", "No outstanding tasks left!".yellow().underline(), "Go for a run :)".green().bold());
                                fs::remove_file(target_file_path.clone()).expect("Failed to find file");
                                // simply removes the local file to prevent an empty vector from being saved
                            } else {
                                let storage_vector_len:usize = storage_vector.len();
                                if storage_vector_len == 1 {
                                    println!("{} {}\n", "You have".yellow(), "1 outstanding task.".yellow().underline());
                                } else {
                                    let storage_vector_len_string:String = storage_vector_len.to_string();
                                    println!("{} {}{}\n", "You have".yellow(), storage_vector_len_string.yellow().underline(), " outstanding tasks.".yellow().underline());
                                }

                                let mut the_save_file = File::create(target_file_path.clone()).expect("File already exists");
                                for eachtask in storage_vector {
                                    let mut task_deadline_string:String = String::from("");
                                    for component in eachtask.task_deadline {
                                        task_deadline_string.push_str(component.to_string().as_str());
                                        task_deadline_string.push_str("/");
                                    };
                                    let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                    match write_to_file_result {
                                        Ok(_) => (),
                                        Err(_) => (),
                                    }
                                }
                                // creates and rewrites the local file with the updated storage_vector
                            }
                        } else {
                            println!("{}\n{}", "No tasks were found.".red().underline(), "Please create a task first".yellow());
                        }
                    },
                    
                    // SORT TASKS
                    "s" => {
                        Command::new("clear").status().expect("Failed to call command");
                        if storage_vector.len() > 0 {
                            println!("{}\n", "Here are your tasks: ".yellow());
                            let mut counter:u8 = 1;
                            for task in &storage_vector {
                                println!("{}. | {:?} ", counter, task.task_name);
                                counter += 1;
                            }
                            println!("\n{}\n{}\n{}\n{}", "Sort tasks by...".yellow(), "[U]rgency".purple(), "D[E]adline".cyan(), "[T]ags".green());
                            let mut sort_criteria:String = String::new();
                            io::stdin().read_line(&mut sort_criteria).expect("Failed to read line");
                            let sort_criteria_str:&str = sort_criteria.as_str().trim_end();
                            match sort_criteria_str {
                                "u" => {
                                    let mut low_urgency_storage_vector:Vec<Task> = vec![];
                                    let mut medium_urgency_storage_vector:Vec<Task> = vec![];
                                    let mut high_urgency_storage_vector:Vec<Task> = vec![];
                                    Command::new("clear").status().expect("Failed to call command");
                                    println!("{} {}", "Sorting by".yellow(), "urgency level.".yellow().underline().bold());
                                    for task in storage_vector {
                                        match task.task_urgency {
                                            UrgencyLevel::Low => {
                                                low_urgency_storage_vector.push(task);
                                            },
                                            UrgencyLevel::Medium => {
                                                medium_urgency_storage_vector.push(task);
                                            },
                                            UrgencyLevel::High => {
                                                high_urgency_storage_vector.push(task);
                                            }, 
                                            // note that there is no need for a match-all statement since an
                                            // enum neccesitates that only its enum variants can fulfill its
                                            // requirements
                                        }
                                    }

                                    counter = 1;

                                    println!("\n{}\n", "High urgency tasks".red());
                                    if high_urgency_storage_vector.is_empty() {
                                        println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline(), "in this category.".yellow());
                                    } else {
                                        display_task_vector(&high_urgency_storage_vector, counter);
                                    };
                                    
                                    println!("\n{}\n", "Medium urgency tasks".blue());
                                    if medium_urgency_storage_vector.is_empty() {
                                        println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                    } else {
                                        display_task_vector(&medium_urgency_storage_vector, counter);
                                    };

                                    println!("\n{}\n", "Low urgency tasks".green());
                                    if low_urgency_storage_vector.is_empty() {
                                        println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline(), "in this category.".yellow());
                                    } else {
                                        display_task_vector(&low_urgency_storage_vector, counter);
                                    };

                                },

                                "e" => {
                                    Command::new("clear").status().expect("Failed to call command");
                                    let mut overdue_storage_vector:Vec<Task> = vec![];
                                    let mut today_storage_vector:Vec<Task> = vec![];
                                    let mut this_month_storage_vector:Vec<Task> = vec![];
                                    let mut next_month_and_later_storage_vector:Vec<Task> = vec![];
                                    let current_day:String = Local::now().to_string();
                                    let current_day_str:&str = current_day.as_str();
                                    let current_day_vector:Vec<&str> = current_day_str.split(" ").collect();
                                    let current_date_vector:Vec<&str> = current_day_vector[0].split("-").collect();
                                    // println!("{:?}", current_date_vector);
                                    let current_year:i32 = current_date_vector[0][2..].parse().expect("Failed to parse number");
                                    let current_month:i32 = current_date_vector[1].parse().expect("Failed to parse number");
                                    let current_day_of_date:i32 = current_date_vector[2].parse().expect("Failed to parse number");
                                    println!("{} {}", "Sorting by".yellow(), "deadline.".yellow().underline());
                                    for task in storage_vector {
                                        if task.task_deadline[2] > current_year {
                                            // deadline is a year or more later
                                            next_month_and_later_storage_vector.push(task);
                                        } else if task.task_deadline[1] > current_month {
                                            // deadline is a month or more later
                                            next_month_and_later_storage_vector.push(task);
                                        } else if task.task_deadline[0] > current_day_of_date {
                                            // deadline is a day or more later
                                            this_month_storage_vector.push(task);
                                        } else if task.task_deadline[0] == current_day_of_date {
                                            // deadline is today
                                            today_storage_vector.push(task);
                                        } else {
                                            // deadline is overdue 
                                            overdue_storage_vector.push(task);
                                        }
                                    };

                                    let counter:u8 = 1;

                                    println!("\n{}\n", "Overdue tasks".red());
                                    if overdue_storage_vector.is_empty() {
                                        println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                    } else {
                                        display_task_vector(&overdue_storage_vector, counter);
                                    };

                                    println!("\n{}\n", "Tasks due today".red());
                                    if today_storage_vector.is_empty() {
                                        println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                    } else {
                                        display_task_vector(&today_storage_vector, counter);
                                    };

                                    println!("\n{}\n", "Tasks due this month".blue());
                                    if this_month_storage_vector.is_empty() {
                                        println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                    } else {
                                        display_task_vector(&this_month_storage_vector, counter);
                                    };

                                    println!("\n{}\n", "Tasks due next month".green());
                                    if next_month_and_later_storage_vector.is_empty() {
                                        println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                    } else {
                                        display_task_vector(&next_month_and_later_storage_vector, counter);
                                    };

                                }, 

                                "t" => {
                                    Command::new("clear").status().expect("Failed to call command");
                                    println!("{} {}\n", "Sorting by".yellow(), "tag type.".yellow().underline());
                                    println!("{}\n", "Here are your tags:".yellow());
                                    let mut tags_collection:Vec<&str> = Vec::new();
                                    for task in &storage_vector {
                                        let indiv_task_tag:Vec<&str> = task.task_tags.split("&").collect();
                                        // println!("{:?}", indiv_task_tag);
                                        for tag in indiv_task_tag {
                                            if tag == " " {
                                            } else {
                                                tags_collection.push(tag);
                                            }
                                        }
                                    }
                                    tags_collection.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                                    // ^ sort a vector
                                    // println!("{:?}", tags_collection);
                                    tags_collection.dedup(); 
                                    // ^ removes duplicates
                                    // println!("{:?}", tags_collection);

                                    if tags_collection.len() > 0 {
                                        let mut counter:u8 = 1;
                                        for tag in &tags_collection {
                                            println!("{}. | {:?} ", counter, tag);
                                            counter += 1;
                                            }
                                        }
                                    println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the tag you would like to sort by:".yellow());
                                    let mut tag_num_to_edit:String = String::new();
                                    io::stdin().read_line(&mut tag_num_to_edit).expect("Failed to read line");
                                    let tag_num_to_edit_int:usize = tag_num_to_edit.trim_end().parse::<usize>().expect("Failed to parse integer of usize") - 1;
                                    println!("Index of the tag to sort by: {}\nTag to sort by: {}", tag_num_to_edit_int, tags_collection[tag_num_to_edit_int]);
                                    // ----- ^ for debugging purposes
                                    Command::new("clear").status().expect("Failed to call command");
                                    let tag_sort_criteria:&str = tags_collection[tag_num_to_edit_int];
                                    let mut sorted_task_tag_collection:Vec<&Task> = Vec::new();
                                    for i in 0..storage_vector.len() {
                                        let given_task:&Task = &storage_vector[i];
                                        let indiv_task_tag:Vec<&str> = storage_vector[i].task_tags.split("&").collect();
                                        // println!("{:?}", indiv_task_tag);
                                        for tag in indiv_task_tag {
                                            if tag == tag_sort_criteria {
                                                sorted_task_tag_collection.push(given_task);
                                            } else {
                                            }
                                        }
                                    }
                                    // println!("{:?}", sorted_task_tag_collection);
                                    println!("{} {} {}\n", "Sorting by the".yellow(), "tag:".yellow().underline(), tag_sort_criteria.green());
                                    let mut counter:u8 = 1;
                                    for task in sorted_task_tag_collection {
                                        println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
                                        println!("{} {}", "Name: ".yellow(), task.task_name);
                                        println!("{} {}", "Description: ".yellow(), task.task_description);
                                        let mut task_deadline_string:String = String::from("");
                                        for component in task.task_deadline {
                                            task_deadline_string.push_str(component.to_string().as_str());
                                            task_deadline_string.push_str("/");
                                        };
                                        task_deadline_string.pop();
                                        println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
                                        if task.task_tags == String::from(" ") {
                                            println!("{} {}\n", "Urgency: ".yellow(), task.task_urgency);
                                        } else {
                                            let task_tags_collection:Vec<&str> = task.task_tags.split("&").collect();
                                            let mut task_tags_for_reader:String = String::new();
                                            for item in task_tags_collection {
                                                task_tags_for_reader.push_str(item);
                                                task_tags_for_reader.push_str(", ");
                                            }
                                            task_tags_for_reader.pop();
                                            task_tags_for_reader.pop();
                                            println!("{} {}", "Urgency: ".yellow(), task.task_urgency);
                                            println!("{} {}\n", "Tags: ".yellow(), task_tags_for_reader);
                                        }
                                        counter += 1;
                                    }
                                },

                                _ => (),
                                // match-all statement
                            }
                        } else {
                            println!("{}\n{}", "No tasks were found.".red().underline(), "Please create a task first".yellow());
                        }
                    },
                    
                    // match-all statement for other cases
                    &_ => {
                        Command::new("clear").status().expect("Failed to call command");
                        println!("{}\n{}", "Invalid input detected.".red().underline(), "Please give a valid input.".yellow());
                    }
                }
            }
        },

        "t" => {
            // global variables
            let mut storage_vector:Vec<Task> = vec![];

            // -----
            
            // printing of logo 
            Command::new("clear").status().expect("Failed to call command");

            // reading of local file and parsing it into the struct Task
            let file_contents_results = fs::read_to_string(".kelpStorage");
            let _file_contents = match file_contents_results {
                Ok(string) => {
                    println!("{}\nLoading data.", "Save file found.".green().underline());
                    let file_contents_array = string.trim_end().split("\n");
                    let file_contents_vector:Vec<&str> = file_contents_array.collect();
                    for eachtask in &file_contents_vector {
                        if eachtask.chars().last().expect("Failed to find character") == ',' {
                            let each_task_array:Vec<&str> = eachtask.split(", ").collect();
                            let each_task_deadline_array:Vec<&str> = each_task_array[2].trim_end_matches("/").split("/").collect();
                            let each_task_deadline:[i32;3] = [each_task_deadline_array[0].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[1].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[2].trim_end().parse().expect("Failed to parse number")];
                            let mut each_task_urgency_unedited:String = each_task_array[3].to_string();
                            each_task_urgency_unedited.truncate(each_task_urgency_unedited.len() -1);
                            match each_task_urgency_unedited.parse::<UrgencyLevel>() {
                                Ok(level) => {
                                    let each_task_urgency:UrgencyLevel = level;
                                    let the_given_task = Task {
                                        task_name: String::from(each_task_array[0]),
                                        task_description: String::from(each_task_array[1]),
                                        task_deadline: each_task_deadline,
                                        task_urgency: each_task_urgency,
                                        task_tags: String::from(" "),
                                        };
                                    storage_vector.push(the_given_task);
                                },
                                Err(_) => (),
                            }
                        } else {
                            let each_task_array:Vec<&str> = eachtask.split(", ").collect();
                            let each_task_deadline_array:Vec<&str> = each_task_array[2].trim_end_matches("/").split("/").collect();
                            let each_task_deadline:[i32;3] = [each_task_deadline_array[0].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[1].trim_end().parse().expect("Failed to parse number"), each_task_deadline_array[2].trim_end().parse().expect("Failed to parse number")];
                            match each_task_array[3].parse::<UrgencyLevel>() {
                                Ok(level) => {
                                    let each_task_urgency:UrgencyLevel = level;
                                    let the_given_task = Task {
                                        task_name: String::from(each_task_array[0]),
                                        task_description: String::from(each_task_array[1]),
                                        task_deadline: each_task_deadline,
                                        task_urgency: each_task_urgency,
                                        task_tags: String::from(each_task_array[4]),
                                        };
                                    storage_vector.push(the_given_task);
                                },
                                Err(_) => (),
                            }
                        }
                    }
                    // for debugging purposes only, to be edited out in actual program
                    println!("{}\n", "Here are your tasks:".yellow());
                    display_task_vector(&storage_vector, 1);
                },
                Err(_) => println!("{}\n{}\n", "No save file found.".red().underline(), "Loading a fresh save.".yellow()),
            };

            // -----

            // menu screen
            println!("{}\n{}\n{}\n{}\n{}", "What would you like to do?".yellow(), "[C]reate new task".magenta(), "[E]dit a task".blue(), "[F]inish a task".cyan(), "[S]ort tasks".bright_green());
            let mut choose_action:String = String::new();
            io::stdin().read_line(&mut choose_action).expect("Failed to read line");
            let choose_action_str:&str = choose_action.as_str().trim_end();

            match choose_action_str {
                
                // CREATE A TASK
                "c" => {

                    Command::new("clear").status().expect("Failed to call command");
                    // create task loop
                    loop {
                        
                        // break condition
                        println!("[E]xit / [Enter] to {}: ", "add task".bold());
                        let mut exit_condition:String = String::new();
                        io::stdin().read_line(&mut exit_condition).expect("Failed to read line");
                        let exit_condition_str:&str = exit_condition.as_str().trim_end();
                        if exit_condition_str == "e" {
                            if storage_vector.len() > 0 {
                                // writing of all tasks to a local file titled .kelpStorage
                                let mut save_file = File::create(".kelpStorage").expect("File already exists");
                                for eachtask in &storage_vector {
                                    let mut task_deadline_string:String = String::from("");
                                    for component in eachtask.task_deadline {
                                        task_deadline_string.push_str(component.to_string().as_str());
                                        task_deadline_string.push_str("/");
                                    };
                                    let write_to_file_result = write!(save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                    match write_to_file_result {
                                        Ok(_) => (),
                                        Err(_) => (),
                                    }
                                }
                            } else {
                                Command::new("clear").status().expect("Failed to call command");
                                println!("{}\n{}", "No tasks were created.".red().underline(), "Exiting without creating save file.".yellow());
                            }
                            break;
                        }

                        // -----

                        // task name
                        Command::new("clear").status().expect("Failed to call command");
                        println!("{} {}{} ", "Enter".yellow(), "task name".yellow().bold(), ":".yellow());
                        let mut userinput_task_name:String = String::new();
                        io::stdin().read_line(&mut userinput_task_name).expect("Failed to read line");
                        let userinput_task_name = String::from(userinput_task_name.trim_end());

                        // -----
                        
                        // task description
                        println!("{} {}{} ", "Enter".yellow(), "task description".yellow().bold(), ":".yellow());
                        let mut userinput_task_description:String = String::new();
                        io::stdin().read_line(&mut userinput_task_description).expect("Failed to read line");
                        let userinput_task_description = String::from(userinput_task_description.trim_end());

                        // -----
                        
                        // task deadline, parsed using destructuring
                        println!("{} {} {} {}{} ", "Enter".yellow(), "task deadline".yellow().bold(), "in the following format".yellow(), "[DD/MM/YY]".underline().yellow(), ":".yellow());
                        let userinput_task_deadline_formatted:[i32; 3];

                        loop {
                            let mut userinput_task_deadline_raw:String = String::new();
                            io::stdin().read_line(&mut userinput_task_deadline_raw).expect("Failed to read line");
                            let userinput_task_deadline_raw_array = userinput_task_deadline_raw.split("/");
                            let userinput_task_deadline_array: Vec<&str> = userinput_task_deadline_raw_array.collect();
                            
                            // checking for valid number of fields input (characters, str literals and numbers covered)
                            if userinput_task_deadline_array.len() != 3 {
                                println!("{}\nEnter {} in the following format {}: ", "Invalid input detected.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                continue;
                            }

                            // checking for characters instead of date input if there are 3 fields
                            if userinput_task_deadline_array[0].chars().all(char::is_numeric) && userinput_task_deadline_array[1].chars().all(char::is_numeric) && userinput_task_deadline_array[2].trim_end().chars().all(char::is_numeric) {
                            } else {
                                println!("{}\nEnter {} in the following format {}: ", "Enter a valid integer input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                continue;
                            }

                            // these have to be signed integers first, to allow for subsequent error checking
                            let userinput_task_deadline_day_int:i32 = userinput_task_deadline_array[0].trim_end().parse().expect("Failed to parse number");
                            let userinput_task_deadline_month_int:i32 = userinput_task_deadline_array[1].trim_end().parse().expect("Failed to parse number");
                            let userinput_task_deadline_year_int:i32 = userinput_task_deadline_array[2].trim_end().parse().expect("Failed to parse number");
                            
                            // checking for valid date inputs
                            if userinput_task_deadline_day_int > 31 || userinput_task_deadline_day_int < 1 {
                                println!("{}\nEnter {} in the following format {}: ", "Enter a valid day input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                continue;
                            }
                            if userinput_task_deadline_month_int > 12 || userinput_task_deadline_month_int < 1 {
                                println!("{}\nEnter {} in the following format {}: ", "Enter a valid month input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                continue;
                            } 
                            if userinput_task_deadline_year_int < 23 || userinput_task_deadline_year_int > 99 {
                                println!("{}\nEnter {} in the following format {}: ", "Enter a valid year input.".red().underline(), "task deadline".bold(), "[DD/MM/YY]".underline());
                                continue; 
                            }
                            userinput_task_deadline_formatted = [userinput_task_deadline_day_int, userinput_task_deadline_month_int, userinput_task_deadline_year_int];
                            break;
                        }
                        
                        // -----

                        // task urgency, handled by an enum
                        println!("{} {} {}{} ", "Enter".yellow(), "task urgency".yellow().bold(), "[L/M/H]".yellow().underline(), ":".yellow());
                        let userinput_task_urgency:UrgencyLevel;
                        
                        loop {
                            let mut userinput_task_urgency_string:String = String::new();
                            io::stdin().read_line(&mut userinput_task_urgency_string).expect("Failed to read line");
                            let userinput_task_urgency_stringliteral:&str = userinput_task_urgency_string.as_str().trim_end();
                            match userinput_task_urgency_stringliteral {
                                "l" => {
                                    userinput_task_urgency = UrgencyLevel::Low;
                                    break;
                                },
                                "m" => {
                                    userinput_task_urgency = UrgencyLevel::Medium;
                                    break;
                                },
                                "h" => {
                                    userinput_task_urgency = UrgencyLevel::High;
                                    break;
                                },
                                // match-all pattern employed for invalid input
                                &_ => {
                                    println!("{} [L/M/H]: ", "Please enter a valid input!".red().underline());
                                    }
                                }
                            }
                        
                        // -----
                        
                        // task tags added to a collection that can then be iterated over
                        println!("{} {}{} {}{}", "Enter".yellow(), "task tags".yellow(), ",".yellow(), "separated by a space".yellow().underline(), ":".yellow());

                        let mut userinput_task_tag:String = String::new();
                        io::stdin().read_line(&mut userinput_task_tag).expect("Failed to read line.");
                        let userinput_task_tag_collection:Vec<&str> = userinput_task_tag.trim_end().split(" ").collect();
                        let userinput_task_tag_collection_formatted:String = userinput_task_tag_collection.join("&");

                        // -----
                        
                        // creation of an instance of the Task struct, and assignment of internal field values
                        let given_task = Task {
                            task_name: userinput_task_name,
                            task_description: userinput_task_description,
                            task_deadline: userinput_task_deadline_formatted,
                            task_urgency: userinput_task_urgency,
                            task_tags: userinput_task_tag_collection_formatted,
                        };
                        
                        // updating of storage_vector:Vec<Task> collection
                        storage_vector.push(given_task);
                        Command::new("clear").status().expect("Failed to call command");

                        };
                        
                    if storage_vector.len() > 0 {
                        Command::new("clear").status().expect("Failed to call command");
                        println!("{}\n", "Here are your tasks: ".yellow());
                        let mut counter:i32 = 1;
                        for task in storage_vector {
                            if task.task_tags == "" || task.task_tags == " " {
                                println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
                                println!("{} {}", "Name: ".yellow(), task.task_name);
                                println!("{} {}", "Description: ".yellow(), task.task_description);
                                let mut task_deadline_string:String = String::from("");
                                for component in task.task_deadline {
                                    task_deadline_string.push_str(component.to_string().as_str());
                                    task_deadline_string.push_str("/");
                                };
                                task_deadline_string.pop();
                                println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
                                println!("{} {}\n", "Urgency: ".yellow(), task.task_urgency);
                            } else {
                                println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
                                println!("{} {}", "Name: ".yellow(), task.task_name);
                                println!("{} {}", "Description: ".yellow(), task.task_description);
                                let mut task_deadline_string:String = String::from("");
                                for component in task.task_deadline {
                                    task_deadline_string.push_str(component.to_string().as_str());
                                    task_deadline_string.push_str("/");
                                };
                                task_deadline_string.pop();
                                println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
                                println!("{} {}", "Urgency: ".yellow(), task.task_urgency);
                                let task_tags_collection:Vec<&str> = task.task_tags.split("&").collect();
                                let mut task_tags_for_reader:String = String::new();
                                for item in task_tags_collection {
                                    task_tags_for_reader.push_str(item);
                                    task_tags_for_reader.push_str(", ");
                                }
                                task_tags_for_reader.pop();
                                task_tags_for_reader.pop();
                                println!("{} {}\n", "Tags: ".yellow(), task_tags_for_reader);
                            }
                                counter += 1;
                        }
                    }
                }, 
                
                // EDIT A TASK
                "e" => {
                    Command::new("clear").status().expect("Failed to call command");
                    // .unwrap() is used for error handling here
                    if storage_vector.len() > 0 {
                        println!("{}\n", "Here are your tasks: ".yellow());
                        let mut counter:u8 = 1;
                        for task in &storage_vector {
                            println!("{}. | {:?} ", counter, task.task_name);
                            counter += 1;
                        }
                        println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the task you would like to edit:".yellow());
                        let mut task_to_edit:String = String::new();
                        io::stdin().read_line(&mut task_to_edit).expect("Failed to read line");
                        let task_to_edit_int:usize = task_to_edit.trim_end().parse::<usize>().expect("Failed to parse integer of usize") - 1;
                        // println!("{}", task_to_edit_int);
                        // println!("Index of the task to be edited: {}", task_to_edit_int);
                        // println!("{:?}", storage_vector[task_to_edit_int].task_name);               
                        // ----- ^ for debugging purposes
                        Command::new("clear").status().expect("Failed to call command");
                        println!("{}\n{}\n{}\n{}\n{}\n{}", "Which component of the task do you want to edit?".yellow(), "[N]ame".purple(), "[D]escription".blue(), "D[E]adline".cyan(), "[U]rgency".bright_green(), "[T]ags".bright_red());
                        let mut what_to_edit:String = String::new();
                        io::stdin().read_line(&mut what_to_edit).expect("Failed to read line");
                        // could use .unwrap() for error handling above as well
                        let what_to_edit_str = what_to_edit.as_str().trim_end();
                        match what_to_edit_str {
                            "n" => {
                                let storage_vector = edit_task_name(task_to_edit_int, storage_vector);
                                let mut the_save_file = File::create(".kelpStorage").expect("File already exists");
                                for eachtask in storage_vector {
                                    let mut task_deadline_string:String = String::from("");
                                    for component in eachtask.task_deadline {
                                        task_deadline_string.push_str(component.to_string().as_str());
                                        task_deadline_string.push_str("/");
                                    };
                                    let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                    match write_to_file_result {
                                        Ok(_) => (),
                                        Err(_) => (),
                                    }
                                }
                            },
                            "d" => {
                                let storage_vector = edit_task_description(task_to_edit_int, storage_vector);
                                let mut the_save_file = File::create(".kelpStorage").expect("File already exists");
                                for eachtask in storage_vector {
                                    let mut task_deadline_string:String = String::from("");
                                    for component in eachtask.task_deadline {
                                        task_deadline_string.push_str(component.to_string().as_str());
                                        task_deadline_string.push_str("/");
                                    };
                                    let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                    match write_to_file_result {
                                        Ok(_) => (),
                                        Err(_) => (),
                                    }
                                }
                            },
                            "e" => {
                                let storage_vector = edit_task_deadline(task_to_edit_int, storage_vector);
                                let mut the_save_file = File::create(".kelpStorage").expect("File already exists");
                                for eachtask in storage_vector {
                                    let mut task_deadline_string:String = String::from("");
                                    for component in eachtask.task_deadline {
                                        task_deadline_string.push_str(component.to_string().as_str());
                                        task_deadline_string.push_str("/");
                                    };
                                    let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                    match write_to_file_result {
                                        Ok(_) => (),
                                        Err(_) => (),
                                    }
                                }
                            },
                            "u" => {
                                let storage_vector = edit_task_urgency(task_to_edit_int, storage_vector);
                                let mut the_save_file = File::create(".kelpStorage").expect("File already exists");
                                for eachtask in storage_vector {
                                    let mut task_deadline_string:String = String::from("");
                                    for component in eachtask.task_deadline {
                                        task_deadline_string.push_str(component.to_string().as_str());
                                        task_deadline_string.push_str("/");
                                    };
                                    let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                    match write_to_file_result {
                                        Ok(_) => (),
                                        Err(_) => (),
                                    }
                                }
                            },
                            "t" => {
                                let storage_vector = edit_task_tags(task_to_edit_int, storage_vector);
                                let mut the_save_file = File::create(".kelpStorage").expect("File already exists");
                                for eachtask in storage_vector {
                                    let mut task_deadline_string:String = String::from("");
                                    for component in eachtask.task_deadline {
                                        task_deadline_string.push_str(component.to_string().as_str());
                                        task_deadline_string.push_str("/");
                                    };
                                    let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                    match write_to_file_result {
                                        Ok(_) => (),
                                        Err(_) => (),
                                    }
                                }
                            }
                            _ => (),
                            // match-all statement
                        };
                    } else {
                        println!("{}\n{}", "No tasks were found.".red().underline(), "Please create a task first".yellow());
                    }
                }, 

                // FINISH A TASK
                "f" => {
                    Command::new("clear").status().expect("Failed to call command");
                    // .unwrap() is used for error handling here
                    if storage_vector.len() > 0 {
                        println!("{}\n", "Here are your tasks: ".yellow());
                        let mut counter:u8 = 1;
                        for task in &storage_vector {
                            println!("{}. | {:?} ", counter, task.task_name);
                            counter += 1;
                        }
                        println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the task you have completed:".yellow());
                        let mut completed_task:String = String::new();
                        io::stdin().read_line(&mut completed_task).expect("Failed to read line");
                        let completed_task_int:usize = completed_task.trim_end().parse::<usize>().expect("Failed to parse integer of usize") - 1;
                        // println!("{}", completed_task_int);
                        Command::new("clear").status().expect("Failed to call command");
                        let removed_task = storage_vector.remove(completed_task_int);
                        println!("{} {}", removed_task.task_name.yellow().underline(), "has been completed!".yellow());
                        println!("{}\n", "Good job! Remember to take breaks and drink enough water!".green());
                        // println!("{:?}", storage_vector);
                        if storage_vector.is_empty() {
                            println!("{}\n{}", "No outstanding tasks left!".yellow().underline(), "Go for a run :)".green().bold());
                            fs::remove_file(".kelpStorage").expect("Failed to find file");
                            // simply removes the local file to prevent an empty vector from being saved
                        } else {
                            let storage_vector_len:usize = storage_vector.len();
                            if storage_vector_len == 1 {
                                println!("{} {}\n", "You have".yellow(), "1 outstanding task.".yellow().underline());
                            } else {
                                let storage_vector_len_string:String = storage_vector_len.to_string();
                                println!("{} {}{}\n", "You have".yellow(), storage_vector_len_string.yellow().underline(), " outstanding tasks.".yellow().underline());
                            }

                            let mut the_save_file = File::create(".kelpStorage").expect("File already exists");
                            for eachtask in storage_vector {
                                let mut task_deadline_string:String = String::from("");
                                for component in eachtask.task_deadline {
                                    task_deadline_string.push_str(component.to_string().as_str());
                                    task_deadline_string.push_str("/");
                                };
                                let write_to_file_result = write!(the_save_file, "{}, {}, {}, {}, {}\n", eachtask.task_name, eachtask.task_description, task_deadline_string, eachtask.task_urgency.to_string(), eachtask.task_tags);
                                match write_to_file_result {
                                    Ok(_) => (),
                                    Err(_) => (),
                                }
                            }
                            // creates and rewrites the local file with the updated storage_vector
                        }
                    } else {
                        println!("{}\n{}", "No tasks were found.".red().underline(), "Please create a task first".yellow());
                    }
                },
                
                // SORT TASKS
                "s" => {
                    Command::new("clear").status().expect("Failed to call command");
                    if storage_vector.len() > 0 {
                        println!("{}\n", "Here are your tasks: ".yellow());
                        let mut counter:u8 = 1;
                        for task in &storage_vector {
                            println!("{}. | {:?} ", counter, task.task_name);
                            counter += 1;
                        }
                        println!("\n{}\n{}\n{}\n{}", "Sort tasks by...".yellow(), "[U]rgency".purple(), "D[E]adline".cyan(), "[T]ags".green());
                        let mut sort_criteria:String = String::new();
                        io::stdin().read_line(&mut sort_criteria).expect("Failed to read line");
                        let sort_criteria_str:&str = sort_criteria.as_str().trim_end();
                        match sort_criteria_str {
                            "u" => {
                                let mut low_urgency_storage_vector:Vec<Task> = vec![];
                                let mut medium_urgency_storage_vector:Vec<Task> = vec![];
                                let mut high_urgency_storage_vector:Vec<Task> = vec![];
                                Command::new("clear").status().expect("Failed to call command");
                                println!("{} {}", "Sorting by".yellow(), "urgency level.".yellow().underline().bold());
                                for task in storage_vector {
                                    match task.task_urgency {
                                        UrgencyLevel::Low => {
                                            low_urgency_storage_vector.push(task);
                                        },
                                        UrgencyLevel::Medium => {
                                            medium_urgency_storage_vector.push(task);
                                        },
                                        UrgencyLevel::High => {
                                            high_urgency_storage_vector.push(task);
                                        }, 
                                        // note that there is no need for a match-all statement since an
                                        // enum neccesitates that only its enum variants can fulfill its
                                        // requirements
                                    }
                                }

                                counter = 1;

                                println!("\n{}\n", "High urgency tasks".red());
                                if high_urgency_storage_vector.is_empty() {
                                    println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline(), "in this category.".yellow());
                                } else {
                                    display_task_vector(&high_urgency_storage_vector, counter);
                                };
                                
                                println!("\n{}\n", "Medium urgency tasks".blue());
                                if medium_urgency_storage_vector.is_empty() {
                                    println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                } else {
                                    display_task_vector(&medium_urgency_storage_vector, counter);
                                };

                                println!("\n{}\n", "Low urgency tasks".green());
                                if low_urgency_storage_vector.is_empty() {
                                    println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline(), "in this category.".yellow());
                                } else {
                                    display_task_vector(&low_urgency_storage_vector, counter);
                                };

                            },

                            "e" => {
                                Command::new("clear").status().expect("Failed to call command");
                                let mut overdue_storage_vector:Vec<Task> = vec![];
                                let mut today_storage_vector:Vec<Task> = vec![];
                                let mut this_month_storage_vector:Vec<Task> = vec![];
                                let mut next_month_and_later_storage_vector:Vec<Task> = vec![];
                                let current_day:String = Local::now().to_string();
                                let current_day_str:&str = current_day.as_str();
                                let current_day_vector:Vec<&str> = current_day_str.split(" ").collect();
                                let current_date_vector:Vec<&str> = current_day_vector[0].split("-").collect();
                                // println!("{:?}", current_date_vector);
                                let current_year:i32 = current_date_vector[0][2..].parse().expect("Failed to parse number");
                                let current_month:i32 = current_date_vector[1].parse().expect("Failed to parse number");
                                let current_day_of_date:i32 = current_date_vector[2].parse().expect("Failed to parse number");
                                println!("{} {}", "Sorting by".yellow(), "deadline.".yellow().underline());
                                for task in storage_vector {
                                    if task.task_deadline[2] > current_year {
                                        // deadline is a year or more later
                                        next_month_and_later_storage_vector.push(task);
                                    } else if task.task_deadline[1] > current_month {
                                        // deadline is a month or more later
                                        next_month_and_later_storage_vector.push(task);
                                    } else if task.task_deadline[0] > current_day_of_date {
                                        // deadline is a day or more later
                                        this_month_storage_vector.push(task);
                                    } else if task.task_deadline[0] == current_day_of_date {
                                        // deadline is today
                                        today_storage_vector.push(task);
                                    } else {
                                        // deadline is overdue 
                                        overdue_storage_vector.push(task);
                                    }
                                };

                                let counter:u8 = 1;

                                println!("\n{}\n", "Overdue tasks".red());
                                if overdue_storage_vector.is_empty() {
                                    println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                } else {
                                    display_task_vector(&overdue_storage_vector, counter);
                                };

                                println!("\n{}\n", "Tasks due today".red());
                                if today_storage_vector.is_empty() {
                                    println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                } else {
                                    display_task_vector(&today_storage_vector, counter);
                                };

                                println!("\n{}\n", "Tasks due this month".blue());
                                if this_month_storage_vector.is_empty() {
                                    println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                } else {
                                    display_task_vector(&this_month_storage_vector, counter);
                                };

                                println!("\n{}\n", "Tasks due next month".green());
                                if next_month_and_later_storage_vector.is_empty() {
                                    println!("{} {} {}", "There are".yellow(), "no tasks".yellow().underline().bold(), "in this category.".yellow());
                                } else {
                                    display_task_vector(&next_month_and_later_storage_vector, counter);
                                };

                            }, 

                            "t" => {
                                Command::new("clear").status().expect("Failed to call command");
                                println!("{} {}\n", "Sorting by".yellow(), "tag type.".yellow().underline());
                                println!("{}\n", "Here are your tags:".yellow());
                                let mut tags_collection:Vec<&str> = Vec::new();
                                for task in &storage_vector {
                                    let indiv_task_tag:Vec<&str> = task.task_tags.split("&").collect();
                                    // println!("{:?}", indiv_task_tag);
                                    for tag in indiv_task_tag {
                                        if tag == " " {
                                        } else {
                                            tags_collection.push(tag);
                                        }
                                    }
                                }
                                tags_collection.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                                // ^ sort a vector
                                // println!("{:?}", tags_collection);
                                tags_collection.dedup(); 
                                // ^ removes duplicates
                                // println!("{:?}", tags_collection);

                                if tags_collection.len() > 0 {
                                    let mut counter:u8 = 1;
                                    for tag in &tags_collection {
                                        println!("{}. | {:?} ", counter, tag);
                                        counter += 1;
                                        }
                                    }
                                println!("\n{} {} {}", "Please enter the".yellow(), "number".yellow().underline(), "of the tag you would like to sort by:".yellow());
                                let mut tag_num_to_edit:String = String::new();
                                io::stdin().read_line(&mut tag_num_to_edit).expect("Failed to read line");
                                let tag_num_to_edit_int:usize = tag_num_to_edit.trim_end().parse::<usize>().expect("Failed to parse integer of usize") - 1;
                                println!("Index of the tag to sort by: {}\nTag to sort by: {}", tag_num_to_edit_int, tags_collection[tag_num_to_edit_int]);
                                // ----- ^ for debugging purposes
                                Command::new("clear").status().expect("Failed to call command");
                                let tag_sort_criteria:&str = tags_collection[tag_num_to_edit_int];
                                let mut sorted_task_tag_collection:Vec<&Task> = Vec::new();
                                for i in 0..storage_vector.len() {
                                    let given_task:&Task = &storage_vector[i];
                                    let indiv_task_tag:Vec<&str> = storage_vector[i].task_tags.split("&").collect();
                                    // println!("{:?}", indiv_task_tag);
                                    for tag in indiv_task_tag {
                                        if tag == tag_sort_criteria {
                                            sorted_task_tag_collection.push(given_task);
                                        } else {
                                        }
                                    }
                                }
                                // println!("{:?}", sorted_task_tag_collection);
                                println!("{} {} {}\n", "Sorting by the".yellow(), "tag:".yellow().underline(), tag_sort_criteria.green());
                                let mut counter:u8 = 1;
                                for task in sorted_task_tag_collection {
                                    println!("{}{}", "Task ".yellow().underline(), counter.to_string().yellow().underline());
                                    println!("{} {}", "Name: ".yellow(), task.task_name);
                                    println!("{} {}", "Description: ".yellow(), task.task_description);
                                    let mut task_deadline_string:String = String::from("");
                                    for component in task.task_deadline {
                                        task_deadline_string.push_str(component.to_string().as_str());
                                        task_deadline_string.push_str("/");
                                    };
                                    task_deadline_string.pop();
                                    println!("{} {}", "Deadline: ".yellow(), task_deadline_string);
                                    if task.task_tags == String::from(" ") {
                                        println!("{} {}\n", "Urgency: ".yellow(), task.task_urgency);
                                    } else {
                                        let task_tags_collection:Vec<&str> = task.task_tags.split("&").collect();
                                        let mut task_tags_for_reader:String = String::new();
                                        for item in task_tags_collection {
                                            task_tags_for_reader.push_str(item);
                                            task_tags_for_reader.push_str(", ");
                                        }
                                        task_tags_for_reader.pop();
                                        task_tags_for_reader.pop();
                                        println!("{} {}", "Urgency: ".yellow(), task.task_urgency);
                                        println!("{} {}\n", "Tags: ".yellow(), task_tags_for_reader);
                                    }
                                    counter += 1;
                                }
                            },

                            _ => (),
                            // match-all statement
                        }
                    } else {
                        println!("{}\n{}", "No tasks were found.".red().underline(), "Please create a task first".yellow());
                    }
                },
                
                // match-all statement for other cases
                &_ => {
                    Command::new("clear").status().expect("Failed to call command");
                    println!("{}\n{}", "Invalid input detected.".red().underline(), "Please give a valid input.".yellow());
                }
            }
            
            // -----
        },
        _ => {
            println!("{}", "Invalid input detected".red().underline());
        }
    }

    }
