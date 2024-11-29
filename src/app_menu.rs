/// Initialize the menus.
pub fn initialize_menus(c: &mut cursive::Cursive) {
    c.menubar()
        .add_subtree(
            "File",
            cursive::menu::Tree::new().leaf("Quit", |c| {
                c.quit();
            }),
        )
        .add_subtree(
            "Edit",
            cursive::menu::Tree::new().leaf("Find", |c| {
                c.add_layer(cursive::views::Dialog::info("Find"))
            }),
        )
        .add_subtree(
            "Help",
            cursive::menu::Tree::new().leaf("About", |c| {
                c.add_layer(
                    cursive::views::Dialog::info("About")
                        //.clear_buttons()
                        .dismiss_button("Close")
                        .button("Ok", |c| {
                            c.pop_layer();
                            c.select_menubar();
                        }),
                )
            }),
        );
    c.set_autohide_menu(false);
    c.add_global_callback(cursive::event::Key::Esc, |c| c.select_menubar());
}
