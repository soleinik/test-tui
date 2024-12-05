use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::sleep,
};

use chrono::Local;
use cursive::{event::Key, views::TextView, CursiveRunnable};

use crate::{
    app_layout,
    app_menu::initialize_menus,
    commands::CtlCommand,
    status::{self, UI_STATUS_TIME},
    table, watchlist,
};

pub struct AppState {
    ticker: Option<String>,
    trade_date: Option<String>,
    tx: CtlSender,
}

impl AppState {
    pub fn ticker(&self) -> Option<String> {
        self.ticker.clone()
    }
    pub fn trade_date(&self) -> Option<String> {
        self.trade_date.clone()
    }

    pub fn send(&mut self, cmd: CtlCommand) {
        self.tx.send(cmd).unwrap();
    }
}

pub struct UI {
    siv: CursiveRunnable,
    rx: Receiver<CtlCommand>,
}

pub type CtlSender = Sender<CtlCommand>;

impl UI {
    pub fn new() -> UI {
        let (tx, rx) = channel::<CtlCommand>();

        cursive::logger::set_internal_filter_level(log::LevelFilter::Info);
        cursive::logger::init();

        let mut siv = cursive::default();
        siv.add_global_callback(Key::F9, cursive::Cursive::toggle_debug_console);

        siv.set_window_title("Trade Helper");

        let state = AppState {
            ticker: None,
            trade_date: None,
            tx,
        };

        siv.set_user_data(state);

        // // Apply a custom theme with a background color
        // let mut theme = siv.current_theme().clone();
        // theme.palette[PaletteColor::Background] = Color::Light(BaseColor::Blue); // Set background color to blue siv.set_theme
        // siv.set_theme(theme);

        let ui = UI { siv, rx };

        ui
    }

    pub fn run(&mut self) {
        let mut runner = self.siv.runner();
        initialize_menus(&mut runner);

        app_layout::app_layout(&mut runner);
        status::create(&mut runner);

        table::create_table(&mut runner, &grid_component::test_table());

        watchlist::watchlist_load(&mut runner);

        // Run the Cursive event loop
        while runner.is_running() {
            while let Some(cmd) = self.rx.try_iter().next() {
                log::info!("cmd: {cmd:?}");
                match cmd {
                    CtlCommand::SelectTicker(ticker) => {
                        log::info!("SelectTicker {}", ticker);
                        //do any broadcasting... if any
                        watchlist::tradedate_load(&mut runner, &ticker);
                        runner.user_data().map(|state: &mut AppState| {
                            state.ticker = Some(ticker);
                        });
                    }
                    CtlCommand::Quit => todo!(),
                    CtlCommand::SelectTradeDate(trade_date) => {
                        log::info!("SelectTradeDate {}", trade_date);
                        runner.user_data().map(|state: &mut AppState| {
                            state.trade_date = Some(trade_date);
                        });
                    }
                }
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
