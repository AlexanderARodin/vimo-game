use anyhow::Result;

use crossterm::event as xEvent;

//  //  //  //  //  //  //  //
pub struct EventHandler {
    rx: std::sync::mpsc::Receiver<Result<Vec<xEvent::Event>>>,
}

impl EventHandler {
    pub fn new(time_out: std::time::Duration) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || loop {
            let Ok(()) = tx.send(collect_events(&time_out)) else {
                break;
            };
        });

        Self { rx }
    }

    pub fn next(&self) -> Result<Vec<xEvent::Event>> {
        match self.rx.try_recv() {
            Ok(ev) => ev,
            Err(std::sync::mpsc::TryRecvError::Empty) => Ok(Vec::new()),
            Err(disconnected) => Err(disconnected.into()),
        }
    }
}

//  //  //  //  //  //  //  //
fn collect_events(poll_wait_time: &std::time::Duration) -> Result<Vec<xEvent::Event>> {
    let mut result = Vec::new();
    while xEvent::poll(*poll_wait_time)? {
        result.push(xEvent::read()?);
    }
    Ok(result)
}
