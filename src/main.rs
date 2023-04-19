use std::{io, thread};
use std::io::Write;
use std::time::Duration;
use crate::format_time::FormatTime;
use crate::timer::Timer;

mod timer;
mod format_time;

fn main() {
    println!("Welcome to the Pomodoro System!");
    let work_duration = get_user_input("Enter the work duration (in minutes)");
    let short_break_duration = get_user_input("Enter the short break duration (in minutes)");
    let long_break_duration = get_user_input("Enter the long break duration (in minutes)");
    println!("Starting Pomodoro...");

    let mut cycles = 0;

    loop {
        let work_timer = Timer::new(work_duration * 60);
        println!("Work time started! ({} minutes)", work_duration);
        run_timer(&work_timer);

        let short_break_timer = Timer::new(short_break_duration * 60);
        println!("Time for a short break! ({} minutes)", short_break_duration);
        run_timer(&short_break_timer);

        cycles += 1;
        if cycles % 4 == 0 {
            let long_break_timer = Timer::new(long_break_duration * 60);
            println!("Time for a long break! ({} minutes)", long_break_duration);
            run_timer(&long_break_timer)
        }

    }
}

fn get_user_input(message: &str) -> u64 {
    loop {
        print!("{}\n>", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse() {
            Ok(n) if n > 0 => return n,
            Ok(_) => println!("Please enter a positive interger number."),
            Err(_) => println!("Invalid input. Please enter a positive integer number."),
        }
    }
}

fn run_timer(timer: &Timer) {
    let total_seconds = timer.duration as f32;
    let mut _progress = 1.0;
    let bar_width = 20;

    while timer.get_time_left() != (0, 0) {
        let (minutes, seconds) = timer.get_time_left();
        let remaining_seconds = (minutes * 60) + seconds;
        let percent_left = (remaining_seconds as f32) / total_seconds;

        let filled_slots = (percent_left * bar_width as f32) as usize;
        let empty_slots = bar_width - filled_slots;

        let progress_bar = format!(
            "[{}{}]",
            "=".repeat(filled_slots),
            "-".repeat(empty_slots)
        );

        print!("\rTime left: {} {}", progress_bar, timer.get_time_left().format_time());
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_secs(1));
        _progress -= 1.0 / total_seconds;
    }

    println!("\nTime's up!");
    timer.wait();
}