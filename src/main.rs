mod timer;
use std::{
    process::exit,
    thread,
    time::{self, Duration, Instant},
};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use notify_rust::Notification;

#[derive(Parser, Debug)]
#[command(
    version, about, long_about = None,
    disable_help_flag = true,
    after_help = "note: must specify a timer duration!\none of -s -m -h must be set.")
]
struct Args {
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// duration of timer in seconds
    #[arg(short, long)]
    seconds: Option<u64>,

    /// duration of timer in minutes
    #[arg(short, long)]
    minutes: Option<u64>,

    /// duration of timer in hours
    #[arg(short, long)]
    hours: Option<u64>,

    /// notification to send when timer is over
    #[arg(short, long)]
    notif: Option<String>,
}

struct Time {
    secs: u64,
    mins: u64,
    hours: u64,
}

impl Time {
    fn from(duration: Duration) -> Self {
        Self {
            secs: duration.as_secs() % 60,
            mins: (duration.as_secs() / 60) % 60,
            hours: duration.as_secs() / 3600,
        }
    }
}

fn format_durations(elapsed: Duration, total: Duration) -> String {
    let elapsed = Time::from(elapsed);
    let total = Time::from(total);

    if elapsed.hours != 0 || total.hours != 0 {
        let elapsed_str = format!(
            "{:0>2}:{:0>2}:{:0>2}",
            elapsed.hours, elapsed.mins, elapsed.secs
        );
        let total_str = format!("{:0>2}:{:0>2}:{:0>2}", total.hours, total.mins, total.secs);
        return format!("{}/{}", elapsed_str, total_str);
    } else {
        let elapsed_str = format!("{:0>2}:{:0>2}", elapsed.mins, elapsed.secs);
        let total_str = format!("{:0>2}:{:0>2}", total.mins, total.secs);
        return format!("{}/{}", elapsed_str, total_str);
    }
}

fn main() {
    let args = Args::parse();
    if args.seconds.is_none() && args.minutes.is_none() && args.seconds.is_none() {
        println!("error: must specify a timer duration!");
        println!("error: one of -s -m -h must be set.");
        exit(1);
    }

    let sty = ProgressStyle::with_template("{msg} [{wide_bar}] ")
        .unwrap()
        .progress_chars("=-_");

    let timer_seconds =
        args.seconds.unwrap_or(0) + 60 * args.minutes.unwrap_or(0) + 3600 * args.hours.unwrap_or(0);
    let total_duration = time::Duration::from_secs(timer_seconds);

    let start_time = Instant::now();

    let steps = 1000.max(timer_seconds);
    let step_duration = total_duration / steps as u32;

    let bar = ProgressBar::new(steps.into()).with_style(sty);
    for _ in 0..steps {
        let cur_time = Instant::now();
        let message = format!(
            "[{}]",
            format_durations(cur_time - start_time, total_duration)
        );
        bar.set_message(message);
        bar.inc(1);
        thread::sleep(step_duration);
    }
    bar.finish();

    // NOTE: how do we bundle this file with the actual application?
    let icon_path = "/home/leaves/Code/rtimer/resources/ktimer.svg";
    let body = args.notif.unwrap_or("time's up!".to_string());
    let _ = Notification::new()
        .summary("rtimer")
        .body(&body)
        .icon(icon_path)
        .show();
}
