use std::fs::OpenOptions;
use std::io::{Read, Write};

// method that calculates the average of the last 5 solves
pub fn get_avg(n: usize) -> f64 {

    // try to open the file
    let mut file = match OpenOptions::new().read(true).open("./src/saves/scrambles.csv") {
        Ok(file) => file,
        Err(_) => return 0.0,
    }; 

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    // collect the times from the file into a vector of f64
    let times: Vec<f64> = contents.lines().map(|x| x.split(",").collect::<Vec<&str>>()[1].parse::<f64>().unwrap()).collect();

    // if there are less then n solves return 0.0 
    if times.len() < n {
        return 0.0;
    }

    let mut last_five: Vec<f64> = times.iter().rev().take(n).map(|x| *x).collect();

    // remove the best and the worst time from the vector
    last_five.sort_by(|a, b| a.partial_cmp(b).unwrap());
    last_five.pop();
    last_five.remove(0);

    // return the average of the last n solves and rounds to 3 decimal places
    ((last_five.iter().sum::<f64>() / (n - 2) as f64) * 1000.0).round() / 1000.0
}

pub fn get_pb() -> f64 {
    // try to open the file
    let mut file = match OpenOptions::new().read(true).open("./src/saves/scrambles.csv") {
        Ok(file) => file,
        Err(_) => return 0.0,
    }; 

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    // collect the times from the file into a vector of f64
    let times: Vec<f64> = contents.lines().map(|x| x.split(",").collect::<Vec<&str>>()[1].parse::<f64>().unwrap()).collect();

    // if there are less then n solves return 0.0 
    if times.len() == 0 {
        return 0.0;
    }

    let x = times.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    x.clone()
}

pub fn get_last() -> f64 {
    // try to open the file
    let mut file = match OpenOptions::new().read(true).open("./src/saves/scrambles.csv") {
        Ok(file) => file,
        Err(_) => return 0.0,
    }; 

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    // collect the times from the file into a vector of f64
    let times: Vec<f64> = contents.lines().map(|x| x.split(",").collect::<Vec<&str>>()[1].parse::<f64>().unwrap()).collect();

    // if there are less then n solves return 0.0 
    if times.len() == 0 {
        return 0.0;
    }

    let x = times.iter().last().unwrap();
    x.clone()
}


pub fn save_data(scramble: &str, time: f64) {
    // try to open the file, if it doesn't exist create it saves/scrambles.txt
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("./src/saves/scrambles.csv")
        .unwrap();

    let data = format!("{},{}\n", scramble.trim_end(), time);
    file.write_all(data.as_bytes()).unwrap();
}
