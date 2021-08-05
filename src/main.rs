use ap::config::Config;
use ap::logging;
use ap::run::run;

fn main() {
    logging::init();
    run(Config::load());
}
