use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = String::from("rsp.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let rounds: Vec<&str> = contents.split("\r\n").collect();
    let results_1: i64 = rounds.iter().map(|x| result(x.split(" ").collect())).sum();
    println!("{:?}", results_1);
    let results_2: i64 = rounds.iter().map(|x| result(get_play(x.split(" ").collect()))).sum();
    println!("{:?}", results_2);

}

fn result(play: Vec<&str>) -> i64 {
    shape(play[1]) + outcome(play)
}

fn outcome(play: Vec<&str>) -> i64 {
    let mut points = HashMap::new();
    let mut points_a = HashMap::new();
    let mut points_b = HashMap::new();
    let mut points_c = HashMap::new();

    points_a.insert("X", 3);
    points_a.insert("Y", 6);
    points_a.insert("Z", 0);
    points.insert("A", points_a);

    points_b.insert("X", 0);
    points_b.insert("Y", 3);
    points_b.insert("Z", 6);
    points.insert("B", points_b);

    points_c.insert("X", 6);
    points_c.insert("Y", 0);
    points_c.insert("Z", 3);
    points.insert("C", points_c);

    *points.get(play[0]).unwrap().get(play[1]).unwrap()
}

fn shape(play: &str) -> i64 {
    let mut shapes = HashMap::new();

    shapes.insert("X", 1);
    shapes.insert("Y", 2);
    shapes.insert("Z", 3);

    *shapes.get(play).unwrap()
}

//tries to retrieve the play to make (using the vocabulary already defined in step 1)
fn get_play(play: Vec<&str>) -> Vec<&str> {
    let mut corr_rules = HashMap::new();
    corr_rules.insert("X", 0);
    corr_rules.insert("Y", 3);
    corr_rules.insert("Z", 6);

    let expected_result: i64 = *corr_rules.get(play[1]).unwrap();
    
    let real_play_option: Vec<bool> = vec!["X", "Y", "Z"].iter().map(|x| outcome(vec![play[0], x]) == expected_result).collect();

    let my_play: &str = vec!["X", "Y", "Z"].iter().zip(real_play_option.iter()).map(|(x, y)| if *y {x} else {"A"}).max().unwrap();
    vec![play[0], my_play]
}