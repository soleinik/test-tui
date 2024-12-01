use std::sync::mpsc::{Receiver, Sender};

use crate::commands::{CtlCommand, UICommand};

pub struct AppController {
    //to UI
    tx: Sender<UICommand>,
    //from UI
    rx: Receiver<CtlCommand>,
}

impl AppController {
    pub fn new(tx: Sender<UICommand>, rx: Receiver<CtlCommand>) -> AppController {
        AppController { tx, rx }
    }

    pub fn run(&self) {
        //messages come from UI, should not ever break - quit only through UI
        while let Ok(msg) = self.rx.recv() {
            match msg {
                CtlCommand::Ready | CtlCommand::WatchList => {
                    self.tx.send(UICommand::WatchList).unwrap();
                }
                CtlCommand::SelectTicker(ticker) => {
                    //println!("{} selected", ticker);
                }
                //For Testing/Debuging
                CtlCommand::Ping(uicommand) => {
                    self.tx.send(uicommand).unwrap();
                }
            }
        }
    }
}
