use std::fs::{OpenOptions, create_dir_all};
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Penalty {
    None,
    PlusTwo,
    DNF,
}

#[derive(Clone, Copy, Debug)]
pub struct Solve {
    pub time: f64,
    pub penalty: Penalty,
}


impl History {
    pub fn new() -> Self {
        Self {
            times: Self::read_times(),
        }
    }

    pub fn save_solve(&mut self, scramble: &str, raw_time: f64, penalty: Penalty) {
        let solve = Solve { time: raw_time, penalty };

        // Update in-memory
        self.times.push(solve);

        // Update file
        if let Err(e) = create_dir_all("./src/saves") {
            eprintln!("Failed to create saves directory: {}", e);
            return;
        }

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("./src/saves/scrambles.csv")
            .unwrap();
        
        // Format: scramble,raw_time,penalty_str
        let p_str = match penalty {
            Penalty::None => "0",
            Penalty::PlusTwo => "2",
            Penalty::DNF => "DNF",
        };

        let data = format!("{},{},{}\n", scramble.trim_end(), raw_time, p_str);
        if let Err(e) = file.write_all(data.as_bytes()) {
            eprintln!("Failed to write to save file: {}", e);
        }
    }

    pub fn get_avg(&self, n: usize) -> f64 {
        if self.times.len() < n {
            return 0.0;
        }

        let last_n: Vec<Solve> = self.times.iter().rev().take(n).copied().collect();
        // Convert to effective times. DNF -> specific handling.
        // Rule: Remove best and worst.
        // For Ao5/Ao12:
        // DNF counts as worst. If >1 DNF, result is DNF (we can represent as -1.0 or similar but typical cube timers might return 0 or specific code).
        // Let's return -1.0 for DNF average.

        // Map solves to values for sorting. DNF should be infinity.
        // +2 should be time + 2.0.
        
        let mut values: Vec<f64> = last_n.iter().map(|s| {
            match s.penalty {
                Penalty::None => s.time,
                Penalty::PlusTwo => s.time + 2.0,
                Penalty::DNF => f64::INFINITY,
            }
        }).collect();

        // Check if more than 1 DNF
        let dnf_count = values.iter().filter(|&&v| v == f64::INFINITY).count();
        if dnf_count > 1 {
            return -1.0; // DNF Average
        }
        
        // Should be safe to sort with Infinity
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Remove best (first) and worst (last)
        // If 1 DNF, it is at the end (worst) and will be removed.
        values.pop();
        values.remove(0);

        let sum: f64 = values.iter().sum();
        let avg = sum / (values.len() as f64);
        
        (avg * 1000.0).round() / 1000.0
    }

    pub fn get_last_five_string(&self) -> String {
        if self.times.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        
        let solves: Vec<Solve> = if self.times.len() < 5 {
            self.times.clone()
        } else {
            self.times.iter().rev().take(5).copied().collect()
        };

        if solves.is_empty() { return String::new(); }

        let values: Vec<f64> = solves.iter().map(|s| {
             match s.penalty {
                Penalty::None => s.time,
                Penalty::PlusTwo => s.time + 2.0,
                Penalty::DNF => f64::INFINITY,
            }
        }).collect();

        let best_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let worst_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        for (i, solve) in solves.iter().enumerate() {
            let val = values[i];
            
            // Format time string
            let time_str = match solve.penalty {
                Penalty::None => format!("{:.2}", solve.time),
                Penalty::PlusTwo => format!("{:.2}+", solve.time + 2.0),
                Penalty::DNF => "DNF".to_string(),
            };

            if val == best_val || val == worst_val {
                result.push_str(&format!("({}) ", time_str));
            } else {
                result.push_str(&format!("{} ", time_str));
            }
        }

        result
    }

    pub fn get_pb(&self) -> f64 {
        if self.times.is_empty() {
            return 0.0;
        }
        
        // Find min single
        self.times.iter()
            .map(|s| match s.penalty {
                Penalty::None => s.time,
                Penalty::PlusTwo => s.time + 2.0,
                Penalty::DNF => f64::INFINITY,
            })
            .fold(f64::INFINITY, |a, b| a.min(b))
    }

    pub fn get_last(&self) -> String {
        if self.times.is_empty() {
            return "0.00".to_string();
        }
        let last = self.times.last().unwrap();
        match last.penalty {
             Penalty::None => format!("{:.2}", last.time),
             Penalty::PlusTwo => format!("{:.2}+", last.time + 2.0),
             Penalty::DNF => "DNF".to_string(),
        }
    }

    fn read_times() -> Vec<Solve> {
        let mut file = match OpenOptions::new().read(true).open("./src/saves/scrambles.csv") {
            Ok(file) => file,
            Err(_) => return Vec::new(),
        }; 
        
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return Vec::new();
        }

        contents.lines()
            .filter_map(|x| {
                let parts: Vec<&str> = x.split(",").collect();
                if parts.len() < 2 {
                    return None;
                }
                
                let time = parts[1].parse::<f64>().ok()?;
                
                // Check if there is a penalty column
                let penalty = if parts.len() > 2 {
                     match parts[2].trim() {
                         "2" => Penalty::PlusTwo,
                         "DNF" => Penalty::DNF,
                         _ => Penalty::None,
                     }
                } else {
                    Penalty::None
                };

                Some(Solve { time, penalty })
            })
            .collect()
    }
}

pub struct History {
    times: Vec<Solve>,
}
