use std::sync::mpsc;
use chrono::prelude::*;
use std::thread;

/**
 * timer - is function which print the local time
 * @noparamter so far
 * Return: void
 */
#[allow(dead_code)]
pub fn timer(sender: mpsc::Sender<String>) {
    thread::spawn(move || {
        loop {
            let local_time = Local::now();
            let formatted_time = format_time_am_pm(local_time);

            // Send the formatted time to the main thread
            sender.send(formatted_time).unwrap();

            // Wait for a second before updating the time
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

/**
 * format_time_am_pm - function which print the format style of the clock
 * @time: dataTime local
 * Return: the string format of the timer
 */
 #[warn(dead_code)]
fn format_time_am_pm(time: DateTime<Local>) -> String {
    let am_pm = if time.hour() < 12 { "AM" } else { "PM" };
    let hour_12 = if time.hour() % 12 == 0 { 12 } else { time.hour() % 12};

    format!("{:02}:{:02}:{:02} {}", hour_12, time.minute(), time.second(), am_pm)
}
