mod cli;
mod port_service;
mod process_service;
mod validation;

use std::process;

fn main() {
    // Parse command line arguments
    let args = match cli::parse_args() {
        Ok(args) => args,
        Err(err) => {
            cli::display_error(&err);
            process::exit(1);
        }
    };

    // Validate the port
    let port = match validation::validate_port(&args.port.to_string()) {
        Ok(port) => port,
        Err(err) => {
            cli::display_error(&err);
            process::exit(1);
        }
    };

    // Query the port to find the process
    let binding = match port_service::find_process_by_port(port) {
        Ok(Some(binding)) => binding,
        Ok(None) => {
            println!("Port {} is not in use", port);
            process::exit(0);
        }
        Err(err) => {
            cli::display_error(&err);
            eprintln!("Hint: Try running as administrator");
            process::exit(1);
        }
    };

    // Get the process name
    let process_name = match process_service::get_process_name(binding.pid) {
        Ok(name) => name,
        Err(err) => {
            cli::display_error(&err);
            eprintln!("Hint: Try running as administrator");
            process::exit(1);
        }
    };

    // Display process information
    cli::display_process_info(binding.pid, &process_name);

    // Terminate the process
    match process_service::kill_process(binding.pid) {
        Ok(()) => {
            cli::display_success(port);
            process::exit(0);
        }
        Err(err) => {
            cli::display_error(&err);
            eprintln!("Hint: Try running as administrator");
            process::exit(1);
        }
    }
}
