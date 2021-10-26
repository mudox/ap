use ap::config::Config;
use ap::logging;
use ap::run::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init();
    run(Config::load());
    Ok(())
}
