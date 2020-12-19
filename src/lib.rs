use std::time::Duration;
use std::thread::sleep;
use std::{env, process};

mod config;

fn set_alarm(time: u64, url: &String) {
    sleep(Duration::new(time, 0));
    open::that(url).unwrap_or_else(|err| {
        println!("Error occured when opening url: {}", err);
        process::exit(1)
    });
}

fn gen_config() -> config::Config {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem occured when parsing the arguments: {}", err);
        process::exit(1)
    });

    config
}

pub fn run() {
    let config::Config {time, url} = gen_config();
    set_alarm(time, &url);
}
