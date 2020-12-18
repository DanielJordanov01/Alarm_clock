use std::time::{Duration, Instant};
use std::thread::sleep;
use std::env;

mod config;

fn set_alarm(time: u64, url: &String) {
    let now = Instant::now();
    sleep(Duration::new(time, 0));
    open::that(url);
}

fn gen_config() -> config::Config {
    let args: Vec<String> = env::args().collect();

    let time = args[1].clone().parse::<u64>().unwrap();
    let url = args[2].clone();

    let config = config::Config::new(time, url);

    config
}

pub fn run() {
    let config = gen_config();
    set_alarm(config.time, &config.url);
}
