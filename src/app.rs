use raylib::prelude::*;
use std::time::Instant;
use crate::scramble_generator;
use crate::scrambler;
use crate::history::{History, Penalty};

pub struct App {
    // Scramble data
    scramble: String,
    cube_state: Vec<Vec<Vec<Color>>>,

    // Timers & Flags
    timer: Instant,
    green_timer: Instant,
    stopwatch_timer: Instant,
    
    // State flags
    is_inspection: bool,
    was_green: bool,
    is_stopwatch: bool,
    is_locked: bool,
    start: bool,
    current_penalty: Penalty,

    // Display
    display_time: String,

    // Stats
    last_five: String,
    avg5: f64,
    avg12: f64,
    avg100: f64,
    pb: f64,

    // Resources
    font: Font,
    history: History,
}

impl App {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // Load resources
        let font = rl.load_font(&thread, "./src/font/JetBrainsMonoNerdFontMono-Bold.ttf")
            .expect("Could not load font");

        // Initial data
        let scramble = scramble_generator::generate();
        let cube_state = scrambler::scramble(&scramble);
        let history = History::new();

        Self {
            scramble,
            cube_state,
            timer: Instant::now(),
            green_timer: Instant::now(),
            stopwatch_timer: Instant::now(),
            
            is_inspection: false,
            was_green: false,
            is_stopwatch: false,
            is_locked: false,
            start: false,
            current_penalty: Penalty::None,

            display_time: history.get_last(),

            last_five: history.get_last_five_string(),
            avg5: history.get_avg(5),
            avg12: history.get_avg(12),
            avg100: history.get_avg(100),
            pb: history.get_pb(),

            font,
            history,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        // Handle Enter: New Scramble
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ENTER) && !self.is_locked {
            self.generate_new_scramble();
        }
        
        // Handle Space: Start Inspection
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) && !self.start && !self.is_stopwatch {
            self.start = true;
            self.timer = Instant::now();
            self.is_inspection = true;
            self.is_locked = true;
            self.current_penalty = Penalty::None;
        }
    
        // Handle P: Cancel inspection
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_P) {
            self.start = false;
        }

        // Logic for Inspection Phase
        if self.start {
            let elapsed = self.timer.elapsed().as_secs();

            if elapsed < 15 {
                self.current_penalty = Penalty::None;
            } else if elapsed < 17 {
                self.current_penalty = Penalty::PlusTwo;
            } else {
                self.current_penalty = Penalty::DNF;
                self.save_and_update_stats(0.0);
                self.generate_new_scramble();
                
                self.start = false;
                self.is_inspection = false;
                self.was_green = false;
                self.is_locked = false;
                return;
            }

            // Key holding logic for separate green/red states
            if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
                if self.is_inspection {
                    self.green_timer = Instant::now();
                    self.is_inspection = false;
                }
                
                if self.green_timer.elapsed().as_secs() >= 1 {
                    self.was_green = true;
                }
            }
           
           // If space released and was green -> Start Timer
           if rl.is_key_up(raylib::consts::KeyboardKey::KEY_SPACE) && self.was_green {
               self.start = false;
               self.is_inspection = false;
               self.was_green = false;
               self.is_stopwatch = true;
               self.stopwatch_timer = Instant::now();
           }
           
           // If space released but wasn't green -> Reset to inspection
           if rl.is_key_up(raylib::consts::KeyboardKey::KEY_SPACE) && !self.was_green {
               self.is_inspection = true;
           }
        }

        // Logic for Stopwatch Phase
        if self.is_stopwatch {
            let elapsed = self.stopwatch_timer.elapsed().as_millis() as f64 / 1000.0;
            self.display_time = format!("{:.3}", elapsed);

            // Stop logic
            if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
                self.is_stopwatch = false;
                self.save_and_update_stats(elapsed);
                self.generate_new_scramble();
                self.is_locked = false;
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        
        // Draw UI
        d.draw_text_ex(&self.font, "Scramble Generator", Vector2::new(10.0, 10.0), 20.0, 0.0, Color::WHITE);
        d.draw_text_ex(&self.font, &self.scramble, Vector2::new(10.0, 30.0), 20.0, 0.0, Color::WHITE);
        
        // Draw Stats
        // Helper to format average nicely
        let fmt_avg = |avg: f64| if avg < 0.0 { "DNF".to_string() } else if avg == 0.0 { "---".to_string() } else { format!("{:.3}", avg) };
        let fmt_pb = |pb: f64| if pb == f64::INFINITY { "DNF".to_string() } else if pb == 0.0 { "---".to_string() } else { format!("{:.3}", pb) };

        d.draw_text_ex(&self.font, format!("PB   :\t{}", fmt_pb(self.pb)).as_str(), Vector2::new(680.0, 10.0), 20.0, 0.0, Color::WHITE);
        d.draw_text_ex(&self.font, format!("Ao5  :\t{}", fmt_avg(self.avg5)).as_str(), Vector2::new(680.0, 30.0), 20.0, 0.0, Color::WHITE);
        d.draw_text_ex(&self.font, format!("Ao12 :\t{}", fmt_avg(self.avg12)).as_str(), Vector2::new(680.0, 50.0), 20.0, 0.0, Color::WHITE);
        d.draw_text_ex(&self.font, format!("Ao100:\t{}", fmt_avg(self.avg100)).as_str(), Vector2::new(680.0, 70.0), 20.0, 0.0, Color::WHITE);
        d.draw_text_ex(&self.font, format!("{}", &self.last_five).as_str(), Vector2::new(20.0, 420.0), 20.0, 0.0, Color::WHITE);

        // Draw Cube
        scrambler::draw_cube(d, self.cube_state.clone(), 0, 0);

        // Draw Timer / Inspection
        let text_pos = Vector2::new(600.0, 225.0);
        let text_size = 50.0;

        if self.start {
            let elapsed = self.timer.elapsed().as_secs();
            
            let display_text = if elapsed < 15 {
                (15 - elapsed).to_string()
            } else if elapsed < 17 {
                "+2".to_string()
            } else {
                "DNF".to_string()
            };

            let mut text_color = Color::WHITE;
            
            if d.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
                if self.was_green { 
                    text_color = Color::GREEN;
                } else { 
                    text_color = Color::RED;
                }
            } else {
                 if elapsed >= 15 { 
                     text_color = Color::RED;
                 }
            };
            
            d.draw_text_ex(&self.font, &display_text, text_pos, text_size, 0.0, text_color);
        } else {
             // Show stopwatch time (running or stopped)
             d.draw_text_ex(&self.font, &self.display_time, text_pos, text_size, 0.0, Color::WHITE);
        }
    }

    fn generate_new_scramble(&mut self) {
        self.scramble = scramble_generator::generate();
        self.cube_state = scrambler::scramble(&self.scramble);
    }

    fn save_and_update_stats(&mut self, time: f64) {
        self.history.save_solve(&self.scramble, time, self.current_penalty);
        
        self.display_time = self.history.get_last();
        
        self.avg5 = self.history.get_avg(5);
        self.avg12 = self.history.get_avg(12);
        self.avg100 = self.history.get_avg(100);
        self.pb = self.history.get_pb();
        self.last_five = self.history.get_last_five_string();
    }
}
