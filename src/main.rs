mod app_controller;
mod app_menu;
mod app_ui;
mod commands;
mod help;
mod status;
mod test_view;
mod watchlist;
use std::sync::mpsc::channel;

use app_controller::AppController;
use app_ui::UI;
use commands::{CtlCommand, UICommand};

fn main() {
    let (ui_tx, ui_rx) = channel::<UICommand>();
    let (c_tx, c_rx) = channel::<CtlCommand>();

    std::thread::spawn(move || UI::new(c_tx, ui_rx).run());

    AppController::new(ui_tx, c_rx).run()
}
