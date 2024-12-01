/// Initialize the menus.
use cursive::{menu::Tree, views::Dialog};

pub fn initialize_menus(c: &mut cursive::Cursive) {
    c.menubar()
        .add_subtree(
            "Lists",
            Tree::new()
                .leaf("Settings...", |siv| {})
                .subtree(
                    "WatchList",
                    Tree::new()
                        .leaf("Maintenance", |s| {
                            //s.add_layer(Dialog::info("WatchList - Maintenance selected"));
                        })
                        .leaf("Snapshot", |s| {
                            //s.add_layer(Dialog::info("WatchList - Snapshot selected"));
                        }),
                )
                .leaf("Portfolio...", |s| {
                    //s.add_layer(Dialog::info("Portfolio selected"));
                })
                .delimiter()
                .leaf("Quit", |s| s.quit()),
        )
        .add_subtree(
            "Edit",
            Tree::new()
                .leaf("Watchlist", |c| {
                    //c.add_layer(cursive::views::Dialog::info("Find"))
                })
                .leaf("Portfolio", |siv| {}),
        )
        .add_delimiter()
        .add_subtree(
            "Max Strikes",
            Tree::new()
                .leaf("Trade Volume", |siv| {})
                .leaf("Implied Volatility", |siv| {})
                .leaf("Open Interest", |siv| {})
                .leaf("Max Pain", |siv| {}),
        )
        .add_subtree(
            "Skews",
            Tree::new()
                .leaf("Trade Volume", |siv| {})
                .leaf("Implied Volatility", |siv| {})
                .leaf("Open Interest", |siv| {}),
        )
        .add_subtree(
            "Comparative",
            Tree::new()
                .leaf("Trade Volume", |siv| {})
                .leaf("Implied Volatility", |siv| {})
                .leaf("Open Interest", |siv| {})
                .leaf("Max Pain", |siv| {}),
        )
        .add_subtree(
            "Research",
            Tree::new()
                .leaf("Expected Move", |siv| {})
                .leaf("Best Calls", |siv| {})
                .leaf("Best Puts", |siv| {})
                .leaf("Summary", |siv| {}),
        )
        .add_delimiter()
        .add_subtree(
            "Help",
            Tree::new().leaf("About", |c| {
                c.add_layer(Dialog::text("About").button("Ok", |c| {
                    c.pop_layer();
                    c.select_menubar();
                }))
            }),
        );
    c.set_autohide_menu(false);
    c.add_global_callback(cursive::event::Key::Esc, |c| c.select_menubar());
}
