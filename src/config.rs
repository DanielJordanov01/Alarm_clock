pub struct Config {
    pub time: u64,
    pub url: String
}

impl Config {
    pub fn new(time: u64, url: String) -> Config {
        Config { time, url }
    }
}