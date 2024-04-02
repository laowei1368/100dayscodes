use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver,select};
use anyhow::Result;


fn main() -> Result<()> {
    let ctrl_c_events = ctrl_channel()?;
    let tickers = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(tickers) -> _ => {
                println!("working");
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

