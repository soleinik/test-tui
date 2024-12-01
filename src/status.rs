use cursive::{
    event::Key,
    theme::{BaseColor, Color},
    view::{Nameable, Resizable},
    views::{FixedLayout, Layer, LinearLayout, OnLayoutView, TextContent, TextView},
    Cursive, Rect, View, With,
};

use crate::{app_ui::CtlSender, commands::CtlCommand, help, watchlist};

pub const UI_STATUS_SYMBOL: &str = "status-symbol";

pub const UI_F1: &str = "status-f1";
pub const UI_F2: &str = "status-f2";
pub const UI_F3: &str = "status-f3";
pub const UI_F4: &str = "status-f4";
pub const UI_F5: &str = "status-f5";
pub const UI_F6: &str = "status-f6";
pub const UI_F7: &str = "status-f7";
pub const UI_F8: &str = "status-f8";
pub const UI_F9: &str = "status-f9";
pub const UI_F10: &str = "status-f10";

pub const UI_STATUS_TIME: &str = "status-time";

const F_KEYS: [&str; 10] = [
    UI_F1, UI_F2, UI_F3, UI_F4, UI_F5, UI_F6, UI_F7, UI_F8, UI_F9, UI_F10,
];

pub fn set_global_help(siv: &mut Cursive) {
    // siv.call_on_name(UI_F1, |tv: &mut TextView| {
    //     tv.set_content("Help");
    // });
    // siv.set_global_callback(Key::F1, |s| {
    //     help::help(s);
    // });

    siv.call_on_name(UI_F2, |tv: &mut TextView| {
        tv.set_content("Watchlist");
    });
    siv.set_global_callback(Key::F2, |s| {
        watchlist::watchlist(s);
        // if let Some(x) = s.user_data::<CtlSender>() {
        //     x.send(CtlCommand::WatchList).unwrap();
        // }
    });

    siv.call_on_name(UI_F3, |tv: &mut TextView| {
        tv.set_content("");
    });

    siv.call_on_name(UI_F4, |tv: &mut TextView| {
        tv.set_content("");
    });

    siv.call_on_name(UI_F5, |tv: &mut TextView| {
        tv.set_content("");
    });
    siv.call_on_name(UI_F6, |tv: &mut TextView| {
        tv.set_content("");
    });
    siv.call_on_name(UI_F7, |tv: &mut TextView| {
        tv.set_content("");
    });
    siv.call_on_name(UI_F8, |tv: &mut TextView| {
        tv.set_content("");
    });
    siv.call_on_name(UI_F9, |tv: &mut TextView| {
        tv.set_content("");
    });
    // siv.call_on_name(UI_F10, |tv: &mut TextView| {
    //     tv.set_content("Quit");
    // });
    // siv.set_global_callback(Key::F10, |s| {
    //     s.quit();
    // });
}

pub fn create(siv: &mut cursive::Cursive) {
    let mut style = cursive::theme::Style::none();
    style.color = cursive::theme::ColorStyle::new(
        Color::Light(BaseColor::Yellow),
        Color::Light(BaseColor::Blue),
    );
    let mut f_style = cursive::theme::Style::none();
    f_style.color = cursive::theme::ColorStyle::new(
        Color::Light(BaseColor::Yellow),
        Color::Dark(BaseColor::Black),
    );

    let views = F_KEYS
        .into_iter()
        .map(|label| {
            TextView::new("")
                .style(style)
                .h_align(cursive::align::HAlign::Left)
                .with_name(label)
                .min_width(8)
                .full_width()
        })
        .collect::<Vec<_>>();

    let symbol_view = TextView::new_with_content(TextContent::new(" -- "))
        .h_align(cursive::align::HAlign::Center)
        .with_name(UI_STATUS_SYMBOL)
        .fixed_width(5);

    let time_view = TextView::new_with_content(TextContent::new("Time"))
        .h_align(cursive::align::HAlign::Left)
        .with_name(UI_STATUS_TIME)
        .fixed_width(8);

    let button_layout = LinearLayout::horizontal()
        .child(
            LinearLayout::horizontal()
                .with(|layout| {
                    for (index, view) in views.into_iter().enumerate() {
                        let letter = TextView::new(format!("F{}", index + 1))
                            .style(f_style)
                            .fixed_width(2);

                        layout.add_child(letter);

                        layout.add_child(view);
                    }
                })
                .full_width(),
        )
        .full_width();

    let button_layer = Layer::with_color(button_layout, style.color).full_width();

    let status_line = LinearLayout::horizontal()
        .child(symbol_view)
        .child(button_layer)
        .child(time_view)
        .full_width();

    let mut status_content_layout = FixedLayout::new();
    status_content_layout.add_child(
        Rect::from_size((0, 0), (siv.screen_size().x, 1)),
        Layer::new(status_line).full_width(),
    );

    let on_layout_view = OnLayoutView::new(status_content_layout, |layout, size| {
        layout.set_child_position(0, Rect::from_size((0, size.y - 1), (size.x, 1)));
        layout.layout(size);
    })
    .full_screen();

    siv.screen_mut().add_transparent_layer(on_layout_view);

    siv.call_on_name(UI_F10, |tv: &mut TextView| {
        tv.set_content("Quit");
    });
    siv.set_global_callback(Key::F10, |s| {
        s.quit();
    });

    siv.call_on_name(UI_F1, |tv: &mut TextView| {
        tv.set_content("Help");
    });
    siv.set_global_callback(Key::F1, |s| {
        help::help(s);
    });
}
