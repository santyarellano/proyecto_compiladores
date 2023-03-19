use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::fs;

enum Production {
    Symbol(String),
    Epsilon
}

impl Display for Production {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Symbol(s) => write!(f, "{}", s),
            Self::Epsilon => write!(f, "*eps*")
        }
    }
}

fn process_str(txt: String, grammar: &mut HashMap<String, Vec<Vec<Production>>>) {
    let lines = txt.lines().collect::<Vec<&str>>();

    // get number of lines to process
    let n_lines: u16 = lines[0].parse().expect("Invalind number of lines");

    // process n lines
    for i in 1..(n_lines + 1) {
        let i: usize = i.into();

        // Scan line
        let mut iter = lines[i].trim_end().chars().peekable();
        let mut peek = iter.next();

        let mut current = String::from("");
        let mut origin = String::from("");
        let mut productions = Vec::new();
        let mut found_arrow = false;

        while peek != None {
            let chr = peek.unwrap();

            match chr {
                ' ' => {
                    let mut skip = false;

                    // check if we're reading epsilon
                    if current == "'" {
                        if iter.peek() != None {
                            if iter.peek().unwrap() == &'\'' {
                                // Epsilon found
                                if found_arrow {
                                    productions.push(Production::Epsilon);
                                    current = "".to_string();
                                    iter.next();
                                    skip = true;
                                }
                                else {
                                    print!("Wrong Input! Epsilon cannot be non terminal.");
                                    return;
                                }
                            }
                        }
                    }

                    // assign origin if not set
                    if current != "" && !skip {
                        if origin == "" {
                            origin = current.clone();
                            current = "".to_string();
                        }
                        else if found_arrow{
                            // Origin exists, append to productions
                            productions.push(Production::Symbol(current.clone()));
                            current = "".to_string();
                        }
                        else {
                            println!("Wrong Line! There can only be one origin.");
                            return;
                        }
                    }
                },
                '-' => {
                    // Check if we're finding an arrow
                    if current.len() == 0 {
                        // this is not part of another symbol
                        if iter.peek() != None {
                            if iter.peek().unwrap() == &'>' {
                                // Arrow found.
                                found_arrow = true;
                                iter.next();
                            }
                            else {
                                // This wasn't an arrow. Add to current.
                                current += &peek.unwrap().to_string();
                            }
                        }
                    }
                    else {
                        // this is part of another symbol
                        current += &peek.unwrap().to_string();
                    }
                },
                _ => {
                    current += &peek.unwrap().to_string();
                },
            }
            
            peek = iter.next();
        }

        // Add last items read
        if current != "" && current != " " {
            if origin == "" {
                println!("Error! Empty non terminal.");
                return;
            }
            else if found_arrow{
                // Origin exists, append to productions
                productions.push(Production::Symbol(current.clone()));
            }
            else {
                println!("Wrong Line! There can only be one origin.");
                return;
            }
        }
        
        // add origin and production to grammar hashmap
        match grammar.entry(origin) {
            Entry::Occupied(mut prods) => {
                prods.get_mut().push(productions);
            }
            Entry::Vacant(entry_prods) => {
                let mut prods = Vec::new();
                prods.push(productions);

                entry_prods.insert(prods);
            }
        }
        
    }
}

fn main() {
    // Ask for file to read
    println!("Enter file path to process: ");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");
    let file_contents = fs::read_to_string(file_path.trim())
        .expect("Error reading file (check file path)");

    // Define grammar hashmap
    let mut grammar: HashMap<String, Vec<Vec<Production>>> = HashMap::new();

    // Process contents of file and store them in the grammar hashmap
    process_str(file_contents, &mut grammar);
    

    // Get terminal and non-terminal symbols
    let mut non_terminals = HashSet::new();
    // add keys to non-terminal
    for (key, _) in &grammar {
        non_terminals.insert(key);
    }

    // add values to terminals if they aren't in non-terminals
    let mut terminals = HashSet::new();
    for (_, value) in &grammar {
        for prods in value {
            for item in prods {
                match item {
                    Production::Epsilon => (),
                    Production::Symbol(s) => {
                        match non_terminals.get(s) {
                            Some(_) => (),
                            None => {
                                // it's a terminal
                                terminals.insert(s);
                            }
                        }
                    }
                }
            }
        }
    }

    // Print terminals
    print!("\nTerminal: ");
    let mut first = true;
    for ter in terminals {
        if !first {
            print!(", ");
        }
        else {
            first = false;
        }
        print!("{ter}");
    }
    println!("");

    // Print non-terminals
    print!("Non terminal: ");
    first = true;
    for nter in non_terminals {
        if !first {
            print!(", ");
        }
        else {
            first = false;
        }
        print!("{nter}");
    }
    println!("");

    // test printing out all grammar
    /*println!("\n------- grammar --------");
    for (key, value) in grammar {
        
        for prods in value {
            print!("{} ({}) -> ", key, prods.len());
            for val in prods {
                print!("{} ", val);
            }
            println!("");
        }
        println!("");
    }*/

}
