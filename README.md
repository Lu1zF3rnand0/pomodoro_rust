# Pomodoro System
The Pomodoro System is a time management technique that helps you stay focused and productive by breaking your workday into 25-minute work sessions, followed by short breaks. After a set number of work sessions, you take a longer break to recharge.

This Pomodoro System is a command-line tool written in Rust that helps you implement the Pomodoro technique in your work routine.

## Getting Started
<ol>
  <li>Install Rust on your machine: <a href="https://www.rust-lang.org/tools/install">https://www.rust-lang.org/tools/install</a></li>
  <li>Clone this repository: <code>git clone https://github.com/&lt;username&gt;/pomodoro-system.git</code></li>
  <li>Navigate to the project directory: <code>cd pomodoro-system</code></li>
  <li>Build the project: <code>cargo build --release</code></li>
  <li>Run the program: <code>./target/release/pomodoro-system</code></li>
</ol>

## Usage
The program will prompt you to enter the work duration, short break duration, and long break duration in minutes. After you enter the durations, the Pomodoro System will start running.

The program will display the remaining time for each work session and break as a progress bar. When the time is up for a work session or break, the program will play a notification sound and wait for you to press Enter to start the next session or break.

To stop the Pomodoro System, press <code>Ctrl+C</code>.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
This project is licensed under the <a href="LICENSE">MIT License</a> - see the <a href="LICENSE">LICENSE</a> file for details.
