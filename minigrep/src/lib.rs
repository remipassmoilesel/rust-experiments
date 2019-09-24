use std::env;

use config::Config;

mod config;

pub fn minigrep() {
    let config = Config::new(env::args().collect());
    
}
