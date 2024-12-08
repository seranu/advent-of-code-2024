use std::env;
use std::process;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::read_to_string;

fn read_to_lines(file_name: &str) -> Vec<String> {
   read_to_string(file_name) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector 
}


fn create_page_rules(lines: &[String]) -> Result<HashMap<i32, HashSet<i32>>, Box<dyn Error>> {
    let mut rules = HashMap::<i32, HashSet<i32>>::new();

    for line in lines {
        if line.len() == 0 {
            break;
        }

        let sep_idx = match line.find('|') {
            Some(i) => i,
            None => return Err("failed to find separator".into()),
        };

        let before: i32 = line[..sep_idx].parse::<i32>()?;
        let after: i32 = line[sep_idx+1..].parse::<i32>()?;

        if !rules.contains_key(&before) {
            rules.insert(before, HashSet::<i32>::new());
        } 

        match rules.get_mut(&before) {
            Some(v) => v.insert(after),
            None => panic!("no element in hashmap"),
        };
    }

    return Ok(rules);
}


fn create_update_list(lines: &[String]) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut updates = Vec::<Vec<i32>>::new();

    for line in lines {
        updates.push(line.split(',')
            .collect::<Vec<_>>()
            .into_iter()
            .map(|nr| nr.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        );
    }
    return Ok(updates);
}

fn is_update_printed(update: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut seen: HashMap<i32, bool> = HashMap::new();
    //println!("Checking: {:?}", update);
    for page in update {
        seen.insert(*page, false);
    }
    for page in update.iter().rev() {
        if rules.contains_key(page) {
            for r in rules.get(page).unwrap() {
                if seen.contains_key(r) && !seen.get(r).unwrap() {
                    //println!("{r} not seen before. Seen: {:?}", seen);
                    return false;
                }
            }
        }

        *seen.get_mut(page).unwrap() = true;
    }

    return true;
}

fn can_add_to_update(page: &i32, seen: &HashMap<i32, bool>,  rules: &HashMap<i32, HashSet<i32>>) -> bool {
    if !rules.contains_key(page) {
        return true;
    }

    for p in rules.get(page).unwrap() {
        if seen.contains_key(p) && !seen.get(p).unwrap() {
            return false;
        }
    }

    return true;
}

fn order_update(update: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut q: VecDeque<i32> = VecDeque::new();
    let mut seen: HashMap<i32, bool> = HashMap::new();
    for page in update {
        seen.insert(*page, false);
        q.push_back(*page);
    }

    let mut ret: Vec<i32> = Vec::new();
    let mut since_last_update = 0;

    while !q.is_empty() {
        let page = q.pop_front().unwrap();
        if can_add_to_update(&page, &seen, rules) {
            ret.push(page);
            *seen.get_mut(&page).unwrap() = true;
            since_last_update = 0;
        } else {
            q.push_back(page);
            since_last_update += 1;
        }

        if since_last_update > q.len() {
            println!("rules: {:?}", rules);
            println!("update: {:?}", update);
            println!("q: {:?}", q);
            println!("ret: {:?}", ret);
            panic!("got stuck reordering");
        }
    }

    return Ok(ret);
}

fn middle_page_sum(filename: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let lines = read_to_lines(filename);

    let sep_idx = lines.iter()
            .position(|el| el.len() == 0)
            .unwrap();

    let rules = create_page_rules(&lines[..sep_idx-1])?;
    let updates = create_update_list(&lines[sep_idx+1..])?;
    let mut sum = 0;
    let mut non_printable_sum = 0;

    for update in &updates {
        if is_update_printed(&update, &rules) {
            sum += update[update.len()/2];
        } else {
            let ordered_update = order_update(&update, &rules).unwrap();
            non_printable_sum += ordered_update[ordered_update.len()/2];
        }
    }

    //println!("Rules {:?}", rules);
    //println!("Updates {:?}", updates);
    return Ok((sum, non_printable_sum));
}

fn main() {
    let args: Vec<String>  = env::args().collect();

    if args.len() != 2 {
        println!("Invalid arguments");
        process::exit(1);
    }

    let ret = middle_page_sum(&args[1]);
    println!("sum= {:?}", ret.unwrap());
}
