use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;

fn process_str(
    txt: String,
    grammar: &mut HashMap<String, Vec<Vec<String>>>,
    first_non_terminal: &mut String,
) {
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
        let mut productions: Vec<String> = Vec::new();
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
                                    productions.push("' '".to_string());
                                    current = "".to_string();
                                    iter.next();
                                    skip = true;
                                } else {
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

                            // assign first non terminal if not set
                            if first_non_terminal == "" {
                                *first_non_terminal = origin.clone();
                            }
                        } else if found_arrow {
                            // Origin exists, append to productions
                            productions.push(current.clone());
                            current = "".to_string();
                        } else {
                            println!("Wrong Line! There can only be one origin.");
                            return;
                        }
                    }
                }
                '-' => {
                    // Check if we're finding an arrow
                    if current.len() == 0 {
                        // this is not part of another symbol
                        if iter.peek() != None {
                            if iter.peek().unwrap() == &'>' {
                                // Arrow found.
                                found_arrow = true;
                                iter.next();
                            } else {
                                // This wasn't an arrow. Add to current.
                                current += &peek.unwrap().to_string();
                            }
                        }
                    } else {
                        // this is part of another symbol
                        current += &peek.unwrap().to_string();
                    }
                }
                _ => {
                    current += &peek.unwrap().to_string();
                }
            }

            peek = iter.next();
        }

        // Add last items read
        if current != "" && current != " " {
            if origin == "" {
                println!("Error! Empty non terminal.");
                return;
            } else if found_arrow {
                // Origin exists, append to productions
                productions.push(current.clone());
            } else {
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

/// Recursive function to get firsts of a non terminal
fn get_firsts(
    grammar: &HashMap<String, Vec<Vec<String>>>,
    terminals: &HashSet<&String>,
    non_terminals: &HashSet<&String>,
    symbol: &String,
    caller: Option<&String>,
) -> HashSet<String> {
    let mut returning_set: HashSet<String> = HashSet::new();

    // base case: this symbol is a terminal or epsilon.
    if terminals.contains(symbol) || symbol == "' '" {
        returning_set.insert(symbol.to_string());
    }
    // this symbol is a non-terminal
    else if non_terminals.contains(symbol) {
        // see first elements in grammar with this symbol
        match grammar.get(symbol) {
            Some(prods) => {
                // iterate over productions to see first elements.
                for prod in prods {
                    // we start checking at 0 (first element)
                    let mut pos = 0;
                    let mut should_continue = true;

                    while should_continue {
                        should_continue = false;

                        // if first element at pos is the same at caller, ignore this iteration to avoid infinite loop
                        match caller {
                            Some(caller_symbol) => {
                                if &prod[pos] == caller_symbol {
                                    continue;
                                }
                            }
                            None => {}
                        }

                        // get firsts of element at pos in production
                        let mut obtained_firsts =
                            get_firsts(grammar, terminals, non_terminals, &prod[pos], Some(symbol));

                        // ***** debug ******
                        // print analisis
                        /*println!("analyzing firsts of {}", prod[pos]);
                        print!("obtained: ");
                        for it in &obtained_firsts {
                            print!("{it}, ");
                        }
                        println!("\n");*/

                        // check if epsilon exists in such firsts
                        if obtained_firsts.contains(&"' '".to_string()) {
                            // epsilon exists in set; is this not the last item?
                            if pos + 1 == prod.len() {
                                // it is the last item, epsilon remains.
                                returning_set.extend(obtained_firsts);
                            } else if pos + 1 < prod.len() {
                                // it is not the last item, remove epsilon and get firsts of next item.
                                obtained_firsts.remove("' '");
                                returning_set.extend(obtained_firsts);
                                pos += 1;
                                should_continue = true;
                            }
                        } else {
                            // epsilon doesn't exist in such firsts
                            returning_set.extend(obtained_firsts);
                        }
                    }
                }
            }
            None => {}
        }
    }

    return returning_set;
}

/// Gets firsts and follows of a given grammar.
/*fn get_firsts_and_follows(
    first_non_terminal: &String,
    terminals: &HashSet<&String>,
    non_terminals: &HashSet<&String>,
) {
    let mut follows: HashMap<String, &HashSet<String>> = HashMap::new();

    // instert $ to follow of first non terminal
    follows.insert(
        first_non_terminal.clone(),
        &HashSet::from_iter(vec!["$".to_string()]),
    );
}
*/

fn main() {
    // Ask for file to read
    println!("Enter file path to process: ");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");
    let file_contents =
        fs::read_to_string(file_path.trim()).expect("Error reading file (check file path)");

    // Define grammar hashmap
    let mut grammar: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    // Process contents of file and store them in the grammar hashmap
    let mut first_non_terminal = "".to_string();
    process_str(file_contents, &mut grammar, &mut first_non_terminal);

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
                if item != "' '" {
                    if !non_terminals.contains(item) {
                        // it's a terminal
                        terminals.insert(item);
                    }
                }
            }
        }
    }

    // Print terminals
    /*print!("\nTerminal: ");
    let mut first = true;
    for ter in terminals {
        if !first {
            print!(", ");
        } else {
            first = false;
        }
        print!("{ter}");
    }
    println!("");*/

    // Print non-terminals
    /*print!("Non terminal: ");
    first = true;
    for nter in non_terminals {
        if !first {
            print!(", ");
        } else {
            first = false;
        }
        print!("{nter}");
    }
    println!("");*/

    // debug print first non terminal
    println!("\n- - -");
    println!("first non terminal: {}", first_non_terminal);
    println!("- - -");

    // print firsts of grammar
    println!("\n- - -");
    for nterm in &non_terminals {
        let firsts = get_firsts(&grammar, &terminals, &non_terminals, nterm, None);
        println!("{nterm}: ");
        print!("FIRST = ");
        for it in firsts {
            print!("{it}, ");
        }
        println!("\n");
    }

    // debug print grammar
    println!("\n- - -");
    for (key, value) in grammar {
        println!("{key}: ");
        for val in value {
            for s in val {
                print!("{s} ")
            }
            println!("");
        }
        println!("");
    }
}
