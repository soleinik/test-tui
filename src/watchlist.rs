use std::sync::mpsc::Sender;

use cursive::align::HAlign;
use cursive::event::{Event, EventResult};
use cursive::views::{Dialog, LinearLayout, OnEventView, ScrollView, SelectView, TextView};
use cursive::Cursive;

use crate::commands::CtlCommand;
use crate::status::UI_STATUS_SYMBOL;

pub fn watchlist(siv: &mut Cursive) {
    let content = include_str!("../watchlist.txt");

    // siv.call_on_name(UI_STATUS_HELP, |tv: &mut TextView| {
    //     tv.set_content("'j':next 'k':-prev 'a':add 'd':del");
    // });

    let select = create_select_view(content, None);

    let layout = LinearLayout::vertical().child(select);

    siv.add_layer(Dialog::around(layout).title("Select Symbol"));
}

fn create_select_view(
    content: &str,
    selected_ticker: Option<&str>,
) -> OnEventView<ScrollView<SelectView>> {
    let mut select = SelectView::new().autojump().h_align(HAlign::Left);
    select.add_all_str(content.lines());

    if let Some(ticker) = selected_ticker {
        if let Some(index) = content.lines().position(|line| line == ticker) {
            select.set_selection(index);
        }
    }

    let select = select
        .on_select(on_select)
        .on_submit(show_confirmation_dialog);

    let select = OnEventView::new(ScrollView::new(select))
        .on_pre_event_inner(Event::Char('k'), |s, _| {
            s.get_inner_mut().select_up(1);
            Some(EventResult::Consumed(None))
        })
        .on_pre_event_inner(Event::Char('j'), |s, _| {
            s.get_inner_mut().select_down(1);
            Some(EventResult::Consumed(None))
        });

    select
}

fn on_select(siv: &mut Cursive, ticker: &String) {
    siv.call_on_name(UI_STATUS_SYMBOL, |tv: &mut TextView| {
        tv.set_content(format!("{ticker}"))
    });

    // siv.set_window_title(format!("Trade Helper[{}]", ticker));
    if let Some(sender) = siv.user_data::<Sender<CtlCommand>>() {
        sender
            .send(CtlCommand::SelectTicker(ticker.to_string()))
            .unwrap();
    }
}

fn show_confirmation_dialog(siv: &mut Cursive, ticker: &str) {
    let ticker = ticker.to_string();
    siv.pop_layer();
    let text = format!("{} is a great symbol!", ticker);
    siv.add_layer(
        Dialog::around(TextView::new(text))
            .button("OK", move |s| {
                s.pop_layer();
                let content = include_str!("../watchlist.txt");
                let select = create_select_view(content, Some(&ticker));
                s.add_layer(Dialog::around(select).title("Select symbol"));
            })
            .button("Quit", |s| s.quit()),
    );
}
