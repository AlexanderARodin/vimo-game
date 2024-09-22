use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

mod app;
use app::App;

//  //  //  //  //  //  //  //
fn main() -> Result<()> {
    log_init();
    debug!("STARTED ----->");

    let terminal = ratatui::init();
    let result = App::new().run(terminal);

    ratatui::restore();
    if let Err(ref e) = result {
        error!("{}", e);
    }

    debug!("<-----");
    trace!("############");
    trace!(" ");
    result
}

//  //  //  //  //  //  //  //
fn log_init() {
    raalog::init().expect("unable init log system")
        .set_file_mode("/tmp/rust_debug.log").expect("unable to set file mode of logger")
        .set_level(raalog::LevelFilter::Trace);

    set_panic_hook();
    trace!(" ");
    trace!("############");
}

//  //  //  //  //  //  //  //
fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        trace!("<-----");
        for line in info.to_string().lines() {
            error!("{}", line);
        }
        error!("############");
        error!(" ");
        hook(info);
    }));
}
