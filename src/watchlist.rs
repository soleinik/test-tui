use cursive::{align::HAlign, views::SelectView, Cursive};

use crate::app_layout::ManageSelectViews;

pub fn watchlist_load(siv: &mut Cursive) {
    let content = routines::watchlist_load("data/watchlist.txt").unwrap();

    let mut select = SelectView::new().autojump().h_align(HAlign::Left);

    let mut lines = content.lines().collect::<Vec<_>>();
    lines.sort();
    select.add_all_str(lines);

    siv.set_view_watchlist(select);
}

pub fn tradedate_load(siv: &mut Cursive, symbol: &str) {
    let content = routines::watchlist_tradedates_for_ticker("data/market", symbol).unwrap();

    let mut select = SelectView::new().autojump().h_align(HAlign::Left);

    let mut lines = content.lines().collect::<Vec<_>>();
    lines.sort();
    lines.reverse();
    select.add_all_str(lines);

    siv.set_view_tradedate(select);
}
