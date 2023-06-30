mod args;

use backtrace::Backtrace;
use std::panic;
use std::time::Instant;
use anyhow::Result;

fn main() -> Result<()> {
    let app_start = Instant::now();
    let cli_args = process_cmdline()?;

    set_panic_handlers()?;

    println!("Hello, world!");

    Ok(())
}

macro_rules! log_eprintln {
    ($string:expr, $e:expr, $bt:expr) => {
        log::error!($string, $e, $bt);
        eprintln!($string, $e, $bt);
    }
}

fn set_panic_handlers() -> Result<()> {
    panic::set_hook(Box::new(|e| {
        let backtrace = Backtrace::new();
        log_eprintln!("panic: {:?}\ntrace\n{:?}", e, backtrace);
        shutdown_terminal();
    }));

    Ok(())
}

fn process_cmdline() -> Result<()> {
    Ok(())
}

fn shutdown_terminal() {

}

