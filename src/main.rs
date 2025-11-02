use std::{process::exit, thread, time};

use clap::Parser;
use indicatif::ProgressBar;
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

fn main() {
    // like what do we actually need to do?
    // would like to spawn a subprocess that
    // sleeps for the appropriate time
    // then sends a notification

    let args = Args::parse();
    if args.seconds.is_none() && args.minutes.is_none() && args.seconds.is_none() {
        println!("error: must specify a timer duration!");
        println!("error: one of -s -m -h must be set.");
        exit(1);
    }

    let timer_seconds =
        args.seconds.unwrap_or(0) + 60 * args.minutes.unwrap_or(0) + 3600 * args.hours.unwrap_or(0);
    let duration = time::Duration::from_secs(timer_seconds);

    let steps = 1000;
    let step_duration = duration / steps;

    let bar = ProgressBar::new(steps.into());
    for _ in 0..steps {
        bar.inc(1);
        thread::sleep(step_duration);
    }
    bar.finish();

    let title = args.notif.unwrap_or("Timer".to_string());
    // NOTE: how do we bundle this file with the actual application?
    let icon_path = "/home/leaves/Code/rtimer/resources/ktimer.svg";
    let _ = Notification::new()
        .summary(&title)
        // .body("notification notification notification!!!")
        .icon(icon_path)
        .show();

    // parent process needs some way to maintain handles to
}
