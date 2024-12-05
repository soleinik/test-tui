pub mod app_layout;
mod app_menu;
mod app_ui;
mod commands;
mod help;
mod status;
mod table;
mod test_view;
mod watchlist;

use app_ui::UI;

fn main() {
    let ui_thread = std::thread::spawn(move || {
        UI::new().run();
    });

    ui_thread.join().unwrap();
}
