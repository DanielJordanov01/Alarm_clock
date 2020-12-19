use std::env;

pub struct Config {
    pub(in crate) time: u64,
    pub(in crate) url: String
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let time = match args.next() {
            Some(arg) => arg.parse::<u64>().unwrap(),
            None => Err("Please enter a number!").unwrap()
        };

        let url = match args.next() {
            Some(arg) => arg,
            None => Err("Please enter a valid url").unwrap()
        };

        Ok(Config {time, url})
    }
}