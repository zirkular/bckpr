mod cli;
mod config;


pub fn run() {
    let yaml = config::read_yaml("config_test.yaml");
    let config = config::parse_config(&yaml);

    match config {
        Ok(config) => {
            for spinoff in config.spinoffs {
                execute_spinoff(&spinoff);
            }
        },
        Err(error) => {
            println!("{:?}", error);
            println!("Configuration could be parsed. Aborting.");
        }
    }
}

pub fn execute_spinoff(spinoff: &config::Spinoff) {
    println!("cli::execute_spinoff()");
    match spinoff.cli {
        config::CLI::Borgmatic => cli::call_borgmatic(),
        config::CLI::Restic => cli::call_restic(),
        config::CLI::Unknown => println!("Unknown CLI. Aborting."),
    }
    
}