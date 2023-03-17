use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::{self, Display, Formatter};
use std::io;

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

fn main() {
    // Read first input (number of lines)
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n_lines: u16 = input.trim().parse().expect("Invalid Input");

    // Define grammar hashmap
    let mut grammar: HashMap<String, Vec<Vec<Production>>> = HashMap::new();

    // Read n lines
    for _ in 0..n_lines {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Scan input string
        let mut iter = input.trim_end().chars().peekable();
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
                //grammar.insert(origin, prods);
            }
        }
        
    }

    // test printing out all grammar
    println!("\n------- grammar --------");
    for (key, value) in grammar {
        
        for prods in value {
            print!("{} ({}) -> ", key, prods.len());
            for val in prods {
                print!("{} ", val);
            }
            println!("");
        }
        println!("");
    }
}
