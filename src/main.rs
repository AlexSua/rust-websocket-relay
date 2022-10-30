mod cli;
mod lock_traits;
mod web_server;

use cli::parse_arguments;
use web_server::web_server_start;

fn main() {
    let cli = parse_arguments();
    let _web_server = match web_server_start(&cli) {
        Ok(result) => result,
        Err(error) => panic!("Problem initializing server: {:?}", error),
    };
}
