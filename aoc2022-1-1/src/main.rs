use std::fs;

fn main() {
    let file_path = String::from("food.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\r\n\r\n").collect();
    let mut sums: Vec<i64> = lines.iter().map(|x| x.split("\r\n").map(|x| x.parse::<i64>().unwrap()).sum()).collect();
    let max = sums.iter().max().unwrap();
    println!("The elf carrying most calories carries {:?}", max);
    sums.sort();
    let top_3_sums: Vec<&i64> = sums.iter().rev().take(3).collect();
    let top_3_total: i64 = top_3_sums.into_iter().sum();
    println!("The elf carrying most calories carries {:?}", top_3_total);
}

