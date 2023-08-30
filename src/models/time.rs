use std::thread;
use std::time::Duration;
use chrono::prelude::*;
/**
 * timer - is function which print the local time
 * @noparamter so far
 * Return: void
 */
pub fn timer() {
    loop {
        let local_time = Local::now();
        let formatted_time = format_time_am_pm(local_time);

        println!("{}", formatted_time);

        // Wait for a second before updating the time
        thread::sleep(Duration::from_secs(1));
    }
}
/**
 * format_time_am_pm - function which print the format style of the clock
 * @time: dataTime local
 * Return: the tring format of the timer
 */
fn format_time_am_pm(time: DateTime<Local>) -> String {
    let am_pm = if time.hour() < 12 { "AM" } else { "PM" };
    let hour_12 = if time.hour() % 12 == 0 { 12 } else { time.hour() % 12};

    format!("{:02}:{:02}:{:02} {}", hour_12, time.minute(), time.second(), am_pm)
}
