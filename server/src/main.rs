mod args;
mod error;
mod serve;

use std::io;
pub use {args::Args, error::Error};

fn main() {
    use {log::error, quit::with_code};

    env_logger::init();

    let code = match result_main() {
        Ok(_) => 0,
        Err(e) => {
            error!("{}", e);
            1
        }
    };

    with_code(code);
}
fn result_main() -> Result<(), io::Error> {
    use {clap::Parser, serve::serve};

    let args = Args::parse();
    serve(args.port, &args.repo)
}
