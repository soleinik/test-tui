use std::{
    sync::mpsc::{Receiver, Sender},
    thread::sleep,
};

use chrono::Local;
use cursive::{views::TextView, CursiveRunnable};

use crate::{
    commands::{CtlCommand, UICommand},
    status::{self, UI_STATUS_TIME},
    test_view,
    watchlist::watchlist,
};

pub struct UI {
    siv: CursiveRunnable,
    rx: Receiver<UICommand>,
    tx: Sender<CtlCommand>,
}

pub type CtlSender = Sender<CtlCommand>;

impl UI {
    pub fn new(tx: CtlSender, rx: Receiver<UICommand>) -> UI {
        let mut siv = cursive::default();
        siv.set_window_title("Trade Helper");
        siv.set_user_data(tx.clone());

        // // Apply a custom theme with a background color
        // let mut theme = siv.current_theme().clone();
        // theme.palette[PaletteColor::Background] = Color::Light(BaseColor::Blue); // Set background color to blue siv.set_theme
        // siv.set_theme(theme);

        UI { siv, rx, tx }
    }

    pub fn run(&mut self) {
        let mut runner = self.siv.runner();
        //runner.set_fps(1);
        // initialize_menus(&mut runner);
        // runner.refresh();

        //UI is ready, notify controller
        self.tx.send(CtlCommand::Ready).unwrap();
        //self.tx.send(CtlCommand::Ping(UICommand::TestView)).unwrap();

        status::create(&mut runner);

        // Run the Cursive event loop
        while runner.is_running() {
            //check for controller's messages, non blocking read
            while let Some(msg) = self.rx.try_iter().next() {
                match msg {
                    UICommand::WatchList => {
                        watchlist(&mut runner);
                    }
                    UICommand::TestView => {
                        test_view::test_view(&mut runner);
                    }
                    UICommand::Quit => todo!(),
                }
                sleep(std::time::Duration::from_millis(1000));
            }

            runner.call_on_name(UI_STATUS_TIME, |tv: &mut TextView| {
                tv.set_content(format!("{}", time()))
            });

            runner.refresh();
            if !runner.step() {
                //sleep(std::time::Duration::from_millis(50));
            }
        }
        drop(runner);
        println!("Quitting UI");
    }
}

pub fn time() -> String {
    let now = Local::now(); // Get local time

    //now.time().format("%H:%M:%S.%3f").to_string()
    now.time().format("%H:%M:%S").to_string()
}
