use std::sync::mpsc::Sender;

use cursive::align::HAlign;
use cursive::event::{Event, EventResult, Key};
use cursive::views::{Dialog, LinearLayout, OnEventView, ScrollView, SelectView, TextView};
use cursive::Cursive;

use crate::app_ui::CtlSender;
use crate::commands::CtlCommand;
use crate::status::{set_global_help, UI_F2, UI_F3, UI_STATUS_SYMBOL};

pub fn watchlist(siv: &mut Cursive) {
    let content = include_str!("../watchlist.txt");

    // siv.call_on_name(UI_STATUS_HELP, |tv: &mut TextView| {
    //     tv.set_content("'j':next 'k':-prev 'a':add 'd':del");
    // });

    let select = create_select_view(content, None);

    let layout = LinearLayout::vertical().child(select);

    add_layer(siv, Dialog::around(layout).title("Select Symbol"));
}

fn add_layer(siv: &mut Cursive, view: Dialog) {
    let view = OnEventView::new(view)
        .on_event(Key::Esc, |siv| {
            pop_layer(siv, None);
        })
        .on_event(Key::F2, |_siv| {
            //add
            //pop_layer(siv, None);
        })
        .on_event(Key::F3, |_siv| {
            //delete
            //pop_layer(siv, None);
        });

    siv.call_on_name(UI_F2, |tv: &mut TextView| {
        tv.set_content("Add");
    });
    siv.call_on_name(UI_F3, |tv: &mut TextView| {
        tv.set_content("Del");
    });

    siv.add_layer(view);
}

fn cb_pop_layer(siv: &mut Cursive, symbol: &str) {
    pop_layer(siv, Some(symbol)) // Some(symbol);
}

fn pop_layer(siv: &mut Cursive, symbol: Option<&str>) {
    siv.pop_layer();

    set_global_help(siv);

    let Some(symbol) = symbol else { return };

    siv.user_data().map(|tx: &mut Option<CtlSender>| {
        if let Some(tx) = tx {
            tx.send(CtlCommand::SelectTicker(symbol.to_owned()))
                .unwrap()
        }
    });
}

fn create_select_view(
    content: &str,
    selected_ticker: Option<&str>,
) -> OnEventView<ScrollView<SelectView>> {
    let mut select = SelectView::new().autojump().h_align(HAlign::Left);

    let mut lines = content.lines().collect::<Vec<_>>();
    lines.sort();
    select.add_all_str(lines);

    if let Some(ticker) = selected_ticker {
        if let Some(index) = content.lines().position(|line| line == ticker) {
            select.set_selection(index);
        }
    }

    let select = select.on_select(on_select).on_submit(cb_pop_layer);

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
