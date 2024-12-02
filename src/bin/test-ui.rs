mod app_layout;

fn main() {
    let mut siv = cursive::default();
    app_layout::app_layout(&mut siv);
    siv.run();
}
