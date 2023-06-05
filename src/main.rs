use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;

#[derive(Eq, Hash, PartialEq, Clone, PartialOrd, Ord)]
struct SlrRule {
    origin: String,
    prod: Vec<String>,
    num: usize,
    is_extended: bool,
}

impl SlrRule {
    fn _new(origin: String, prod: &Vec<String>, rule_number: usize) -> SlrRule {
        let mut new_prod: Vec<String> = Vec::new();
        // add pointer
        new_prod.push("'*'".to_string());
        new_prod.append(&mut prod.clone());
        return SlrRule {
            origin,
            prod: new_prod,
            num: rule_number,
            is_extended: false,
        };
    }

    fn init(&mut self) {
        let mut new_prod = vec!["'*'".to_string()];
        new_prod.append(&mut self.prod.clone());
        self.prod = new_prod;
    }

    fn to_string(&self) -> String {
        let mut ret = "".to_string();
        ret += &(self.num.to_string() + &". ".to_string() + &self.origin + " -> ");
        for prod in self.prod.iter() {
            ret += &(prod.to_owned() + &" ".to_string().to_owned());
        }
        return ret;
    }

    fn get_reading_symbol(&self) -> Option<String> {
        for i in 0..self.prod.len() {
            if self.prod[i] == "'*'" && i < self.prod.len() - 1 {
                return Some(self.prod[i + 1].clone());
            }
        }

        return None;
    }

    fn advance(&mut self) {
        for i in 0..(self.prod.len() - 1) {
            if self.prod[i] == "'*'" {
                self.prod.swap(i, i + 1);
                return;
            }
        }
    }
}

struct SlrState {
    kernel: HashSet<SlrRule>,
    extended_state: HashSet<SlrRule>,
    transitions: HashSet<(String, usize)>,
}

impl SlrState {
    fn new() -> SlrState {
        SlrState {
            kernel: HashSet::new(),
            extended_state: HashSet::new(),
            transitions: HashSet::new(),
        }
    }

    fn get_reading_symbols(&self) -> HashSet<String> {
        let mut symbols: HashSet<String> = HashSet::new();

        // from kernel
        for rule in self.kernel.iter() {
            let symbol = rule.get_reading_symbol();
            match symbol {
                Some(s) => {
                    symbols.insert(s);
                }
                None => {}
            }
        }

        // from extended
        for rule in self.extended_state.iter() {
            let symbol = rule.get_reading_symbol();
            match symbol {
                Some(s) => {
                    symbols.insert(s);
                }
                None => {}
            }
        }

        return symbols;
    }

    fn get_next_kernel(&self, reading_symbol: &String) -> HashSet<SlrRule> {
        let mut new_kernel: HashSet<SlrRule> = HashSet::new();

        // from kernel
        for rule in self.kernel.iter() {
            match rule.get_reading_symbol() {
                Some(symbol) => {
                    if symbol == reading_symbol.clone() {
                        let mut new_rule = rule.clone();
                        new_rule.advance();
                        new_kernel.insert(new_rule);
                    }
                }
                None => {}
            }
        }

        // from extended
        for rule in self.extended_state.iter() {
            match rule.get_reading_symbol() {
                Some(symbol) => {
                    if symbol == reading_symbol.clone() {
                        let mut new_rule = rule.clone();
                        new_rule.advance();
                        new_kernel.insert(new_rule);
                    }
                }
                None => {}
            }
        }

        return new_kernel;
    }

    fn _to_string(&self) -> String {
        let mut ret = "".to_string();

        // add rules from kernel
        for rule in self.kernel.iter() {
            ret += &(rule.to_string() + "\n");
        }
        ret += "- - - - - - -\n";
        // add rules from extended
        for rule in self.extended_state.iter() {
            ret += &(rule.to_string() + "\n");
        }

        return ret;
    }

    fn get_end_rules(&self) -> HashMap<String, usize> {
        let mut ret = HashMap::new();

        for rule in self.kernel.iter() {
            if rule.prod.last().unwrap() == "'*'" {
                ret.insert(rule.origin.clone(), rule.num);
            }
        }

        return ret;
    }
}

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

enum Action {
    S(usize),
    R(usize),
    Acc,
    Err,
}

struct SlrRow {
    actions: HashMap<String, Action>,
    gotos: HashMap<String, usize>,
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

/// Recursive function to get follows of a non terminal
fn get_follows(
    grammar: &HashMap<String, Vec<Vec<String>>>,
    terminals: &HashSet<&String>,
    non_terminals: &HashSet<&String>,
    symbol: &String,
    first_non_terminal: &String,
    caller: Option<&String>,
) -> HashSet<String> {
    let mut returning_set: HashSet<String> = HashSet::new();

    // check if this symbol is the first non terminal.
    if symbol == first_non_terminal {
        returning_set.insert("$".to_string());
    }

    // go through grammar to find this item in right hand of definitions
    for (origin, prods) in grammar {
        for prod in prods {
            for mut pos in 0..prod.len() {
                // check if current item is the symbol we're looking for
                if &prod[pos] == symbol {
                    // we should act until we find a follow that doesn't contain epsilon or end the production.
                    let mut should_repeat_next = true;
                    while should_repeat_next {
                        should_repeat_next = false;

                        // is this the last item?
                        if pos + 1 == prod.len() {
                            // this is the last item, is the origin the caller of this follows?
                            match caller {
                                Some(caller_symbol) => {
                                    if caller_symbol == &prod[pos] {
                                        // this is the caller of this follows, we should avoid recursion. Just continue
                                        break;
                                    }
                                }
                                None => {}
                            }

                            //use follow of origin
                            let origin_follows = get_follows(
                                grammar,
                                terminals,
                                non_terminals,
                                origin,
                                first_non_terminal,
                                Some(symbol),
                            );
                            returning_set.extend(origin_follows);
                        } else {
                            // this is not the last item, act upon next item
                            // check if next item is this same item; if so, just continue
                            if &prod[pos + 1] == &prod[pos] {
                                continue;
                            }
                            // if not, get firsts of next item
                            let mut next_firsts =
                                get_firsts(grammar, terminals, non_terminals, &prod[pos + 1], None);
                            // check if such firsts have epsilon
                            if next_firsts.contains("' '") {
                                // they do, in such case we concatenate (without epsilon) and do the same for next item.
                                next_firsts.remove("' '");
                                returning_set.extend(next_firsts);
                                should_repeat_next = true;
                                pos += 1;
                            } else {
                                // they don't, we should just add such firsts
                                returning_set.extend(next_firsts);
                            }
                        }
                    }
                }
            }
        }
    }

    return returning_set;
}

fn _print_grammar(grammar: &HashMap<String, Vec<Vec<String>>>) {
    println!("\n- - -");
    println!("GRAMMAR\n");
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

fn print_extended_grammar(extended_grammar: &Vec<SlrRule>) {
    println!("\n- - -");
    println!("EXTENDED GRAMMAR\n");
    for rule in extended_grammar {
        println!("{}", rule.to_string());
    }
}

fn print_firsts_follows(
    grammar: &HashMap<String, Vec<Vec<String>>>,
    non_terminals: &HashSet<&String>,
    terminals: &HashSet<&String>,
    first_non_terminal: &String,
) {
    println!("\n- - -");
    println!("FIRSTS & FOLLOWS \n");
    for nterm in non_terminals {
        // firsts
        let firsts = get_firsts(&grammar, &terminals, &non_terminals, nterm, None);
        println!("{nterm}: ");
        print!("FIRST = ");
        for it in firsts {
            print!("{it}, ");
        }

        // follows
        print!("\nFOLLOW = ");
        let follows = get_follows(
            &grammar,
            &terminals,
            &non_terminals,
            nterm,
            &first_non_terminal,
            None,
        );
        for it in follows {
            print!("{it}, ");
        }
        println!("\n");
    }
}

fn get_extended_prods(extended_grammar: &Vec<SlrRule>, key: String) -> HashSet<SlrRule> {
    let mut prods: HashSet<SlrRule> = HashSet::new();
    for rule in extended_grammar.iter() {
        if rule.origin == key {
            let mut new_rule = rule.clone();
            new_rule.init();
            prods.insert(new_rule);
        }
    }

    return prods;
}

fn add_extender_prods(extended_grammar: &Vec<SlrRule>, state: &mut SlrState) {
    let current_symbols = state.get_reading_symbols();
    for symbol in current_symbols.iter() {
        for rule in extended_grammar.iter() {
            if rule.origin == symbol.clone() {
                let mut new_rule = rule.clone();
                new_rule.init();
                state.extended_state.insert(new_rule);
            }
        }
    }
}

fn _print_slr(slr: &Vec<SlrState>) {
    println!("\n- - -");
    println!("SLR\n");
    for i in 0..slr.len() {
        println!("I{}:", i);
        println!("{}\n", slr[i]._to_string());
    }
}

fn _kernel_to_string(kernel: &HashSet<SlrRule>) -> String {
    let mut ret = "".to_string();

    for rule in kernel.iter() {
        ret += &(rule.to_string());
    }

    return ret;
}

fn insert_to_kernels_hash(
    hash: &mut HashMap<Vec<SlrRule>, usize>,
    kernel: &HashSet<SlrRule>,
    idx: usize,
) {
    let mut kernel_to_add = Vec::from_iter(kernel.clone());
    kernel_to_add.sort();
    hash.insert(kernel_to_add, idx);
}

fn build_slr(slr: &mut Vec<SlrState>, extended_grammar: &Vec<SlrRule>) {
    let mut slr_len = 0;
    let mut states_to_build: HashSet<usize> = HashSet::new();
    let mut kernels: HashMap<Vec<SlrRule>, usize> = HashMap::new();

    // add state 0
    let mut rule0 = extended_grammar[0].clone();
    rule0.init();
    let mut kernel0: HashSet<SlrRule> = HashSet::new();
    kernel0.insert(rule0.clone());
    // add kernel 0 to kernels hashmap
    insert_to_kernels_hash(&mut kernels, &kernel0, 0);
    let extended_prods = get_extended_prods(extended_grammar, rule0.get_reading_symbol().unwrap());
    slr.push(SlrState {
        kernel: kernel0,
        extended_state: extended_prods,
        transitions: HashSet::new(),
    });
    slr_len += 1;

    // create state 0 transitions
    let reading_symbols = slr[0].get_reading_symbols();
    for symbol in reading_symbols.iter() {
        // create new kernel advancing under such symbols
        let new_kernel = slr[0].get_next_kernel(symbol);
        insert_to_kernels_hash(&mut kernels, &new_kernel, slr_len);
        let mut new_state = SlrState::new();
        new_state.kernel = new_kernel;
        slr[0].transitions.insert((symbol.clone(), slr_len));
        slr.push(new_state);
        states_to_build.insert(slr_len);
        slr_len += 1;
    }

    // create rest of the states
    while !states_to_build.is_empty() {
        let next_state_option = states_to_build.iter().next();
        let mut state_idx: Option<usize> = None;
        match next_state_option {
            Some(idx) => {
                state_idx = Some(idx.clone());
            }
            None => {}
        }
        if state_idx.is_some() {
            let idx = state_idx.unwrap();
            // create this state
            // init extended productions
            add_extender_prods(extended_grammar, &mut slr[idx]);
            // transitions
            let next_symbols = slr[idx].get_reading_symbols();
            for symbol in next_symbols.iter() {
                // get new kernel
                let mut new_kernel = Vec::from_iter(slr[idx].get_next_kernel(symbol));
                new_kernel.sort();
                // check if kernel already exists
                match kernels.get(&new_kernel) {
                    Some(existing_idx) => {
                        // just add a transition for this found state
                        slr[idx].transitions.insert((symbol.clone(), *existing_idx));
                    }
                    None => {
                        // create new state
                        // create new kernel advancing under such symbols and add transition
                        let new_kernel = slr[idx].get_next_kernel(symbol);
                        insert_to_kernels_hash(&mut kernels, &new_kernel, slr_len);
                        let mut new_state = SlrState::new();
                        new_state.kernel = new_kernel;
                        slr[idx].transitions.insert((symbol.clone(), slr_len));
                        slr.push(new_state);
                        states_to_build.insert(slr_len);
                        slr_len += 1;
                    }
                }
            }
            // remove this idx from statest to build
            states_to_build.remove(&idx);
        }
    }
}

fn build_slr_table(
    slr: &Vec<SlrState>,
    table: &mut Vec<SlrRow>,
    terminals: &HashSet<&String>,
    non_terminals: &HashSet<&String>,
    grammar: &HashMap<String, Vec<Vec<String>>>,
    first_non_terminal: &String,
) {
    for state in slr.iter() {
        let mut row: SlrRow = SlrRow {
            actions: HashMap::new(),
            gotos: HashMap::new(),
        };

        // act upong transitions
        for transition in state.transitions.iter() {
            // if non terminal add goto
            if non_terminals.contains(&transition.0) {
                row.gotos.insert(transition.0.clone(), transition.1.clone());
            }
            // if terminal add s
            else if terminals.contains(&transition.0) {
                row.actions
                    .insert(transition.0.clone(), Action::S(transition.1.clone()));
            }
        }

        // act if state has end of reading (pointer at the end of production)
        let ending_rules = state.get_end_rules();
        if !ending_rules.is_empty() {
            // add reduce for each follow of ending rule
            for rule in ending_rules.iter() {
                let follows = get_follows(
                    grammar,
                    terminals,
                    non_terminals,
                    rule.0,
                    first_non_terminal,
                    None,
                );

                for symbol in follows {
                    match row.actions.get_mut(&symbol) {
                        Some(v) => {
                            // set error if row action already exists
                            *v = Action::Err;
                        }
                        None => {
                            // add reduce
                            if rule.1.clone() == 0 as usize {
                                row.actions.insert(symbol, Action::Acc);
                            } else {
                                row.actions.insert(symbol, Action::R(rule.1.clone()));
                            }
                        }
                    }
                }
            }
        }

        table.push(row);
    }
}

fn slr_table_to_string(
    table: &Vec<SlrRow>,
    non_terminals: &HashSet<&String>,
    terminals: &HashSet<&String>,
) -> String {
    let mut ret = "<table>".to_string();

    // add headers
    ret += "<tr>";
    ret += "<th>state</th>";
    for term in terminals.iter() {
        ret += "<th>";
        ret += term.clone();
        ret += "</th>";
    }
    ret += "<th>$</th>";
    for nterm in non_terminals.iter() {
        ret += "<th>";
        ret += nterm.clone();
        ret += "</th>";
    }
    ret += "</tr>";

    // add rows
    for i in 0..table.len() {
        ret += "<tr>";

        // state id
        ret += "<td>";
        ret += &i.to_string();
        ret += "</td>";

        // contents of rows
        //      actions
        for term in terminals.iter() {
            ret += "<td>";
            match table[i].actions.get(term.clone()) {
                Some(action) => match action {
                    Action::Acc => {
                        ret += "ACC";
                    }
                    Action::Err => {
                        ret += "ERR";
                    }
                    Action::R(r) => {
                        ret += "r";
                        ret += &r.to_string();
                    }
                    Action::S(s) => {
                        ret += "s";
                        ret += &s.to_string();
                    }
                },
                None => {}
            }
            ret += "</td>";
        }
        //      action ($)
        ret += "<td>";
        match table[i].actions.get("$") {
            Some(action) => match action {
                Action::Acc => {
                    ret += "ACC";
                }
                Action::Err => {
                    ret += "ERR";
                }
                Action::R(r) => {
                    ret += "r";
                    ret += &r.to_string();
                }
                Action::S(s) => {
                    ret += "s";
                    ret += &s.to_string();
                }
            },
            None => {}
        }
        ret += "</td>";
        //      gotos
        for nterm in non_terminals.iter() {
            ret += "<td>";
            match table[i].gotos.get(nterm.clone()) {
                Some(goto) => {
                    ret += &goto.to_string();
                }
                None => {}
            }
            ret += "</td>";
        }

        ret += "</tr>";
    }

    ret += "</table>";
    return ret;
}

fn main() {
    // Choose whether to use a file or input the grammar
    let _use_file = true;
    let mut txt = String::new();
    if !_use_file {
        println!("Enter grammar (starting with number of rules): ");
        io::stdin()
            .read_line(&mut txt)
            .expect("Failed to read line");
        // parse it to number
        let n_rules: u16 = txt.trim().parse().expect("Invalind number of lines");
        for _ in 0..n_rules {
            io::stdin()
                .read_line(&mut txt)
                .expect("Failed to read line");
        }
    } else {
        println!("Enter filename: ");
        let mut file_path = String::new();
        io::stdin()
            .read_line(&mut file_path)
            .expect("Failed to read line");
        txt = fs::read_to_string(file_path.trim()).expect("Error reading file (check file path)");
    }

    // Define grammar hashmap
    let mut grammar: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    // Process contents of file and store them in the grammar hashmap
    let mut first_non_terminal = "".to_string();
    process_str(txt, &mut grammar, &mut first_non_terminal);

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
    /*println!("\n- - -");
    println!("first non terminal: {}", first_non_terminal);
    println!("- - -");*/

    //print_grammar(&grammar);

    print_firsts_follows(&grammar, &non_terminals, &terminals, &first_non_terminal);

    // = = = SLR = = =
    let mut extended_grammar: Vec<SlrRule> = Vec::new();
    //      add extended grammar rule
    extended_grammar.push(SlrRule {
        origin: first_non_terminal.clone() + "'",
        prod: vec![first_non_terminal.clone()],
        num: 0,
        is_extended: true,
    });
    //      add rules
    for item in grammar.iter() {
        for prod in item.1.iter() {
            extended_grammar.push(SlrRule {
                origin: item.0.clone(),
                prod: prod.clone(),
                num: extended_grammar.len(),
                is_extended: false,
            })
        }
    }

    print_extended_grammar(&extended_grammar);

    let mut slr: Vec<SlrState> = Vec::new();
    build_slr(&mut slr, &extended_grammar);

    _print_slr(&slr);

    let mut slr_table: Vec<SlrRow> = Vec::new();
    build_slr_table(
        &slr,
        &mut slr_table,
        &terminals,
        &non_terminals,
        &grammar,
        &first_non_terminal,
    );

    let table_html = slr_table_to_string(&slr_table, &non_terminals, &terminals);
    println!("\n{}\n", table_html);

    //print_slr(&slr);
}
