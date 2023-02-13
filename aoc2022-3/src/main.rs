use std::fs;
use std::collections::HashSet;

fn main() {
    let mut alph = String::from("abcdefghijklmnopqrstuvwxyz");
    alph.push_str(&alph.to_uppercase());

    let file_path = String::from("ruck.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sacks: Vec<&str> = contents.split("\r\n").collect();
    let priority: usize = sacks.iter().map(|sack| compute_priority(&alph, get_duplicate(get_compartments(sack)))).sum();
    println!("{}", priority);

    let priority_group_badges : usize = sacks.chunks(3).collect::<Vec<&[&str]>>().iter().map(|grp| compute_priority(&alph, find_badge(grp))).sum();
    println!("{:?}", priority_group_badges);
}

fn get_compartments(sack: &str) -> (&str, &str) {
    let l = sack.len()/2;
    (&sack[..l], &sack[l..])
}

fn get_duplicate((first, second): (&str, &str)) -> char {
    for letter in first.chars() {
        if second.contains(letter) {
            return letter;
        }
    }
    panic!("Expected duplicate to exist!");
}

fn compute_priority(alph: &str, c: char) -> usize {
    alph.find(c).unwrap() + 1
}

fn find_badge(group: &[&str]) -> char {
    
    let mut a = HashSet::new();
    group[0].chars().for_each(|x| {a.insert(x);});

    let mut b = HashSet::new();
    group[1].chars().for_each(|x| {b.insert(x);});

    let mut c = HashSet::new();
    group[2].chars().for_each(|x| {c.insert(x);});

    let common_char = a.iter().filter(|e| b.contains(e) && c.contains(e)).collect::<Vec<&char>>();
    **common_char.first().unwrap()
 
}