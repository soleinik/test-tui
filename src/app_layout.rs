use cursive::{
    align::HAlign,
    views::{LinearLayout, ResizedView, SelectView, TextView},
};

use crate::watchlist;

pub(crate) fn app_layout(siv: &mut cursive::Cursive) {
    let mut watchlist = SelectView::new().autojump().h_align(HAlign::Left);
    watchlist.add_item_str("NVDA");
    watchlist.add_item_str("FBTC");
    watchlist.add_item_str("WULF");
    watchlist.add_item_str("MARA");
    watchlist.add_item_str("HIMS");
    watchlist.add_item_str("IREN");
    watchlist.add_item_str("HUT");
    watchlist.add_item_str("^SPX");

    let mut trade_date_view = SelectView::new().autojump().h_align(HAlign::Left);
    trade_date_view.add_item_str("2020-01-01");
    trade_date_view.add_item_str("2020-02-01");
    trade_date_view.add_item_str("2020-03-01");
    trade_date_view.add_item_str("2020-04-01");
    trade_date_view.add_item_str("2020-05-01");
    trade_date_view.add_item_str("2020-06-01");
    trade_date_view.add_item_str("2020-07-01");
    trade_date_view.add_item_str("2020-08-01");
    trade_date_view.add_item_str("2020-09-01");
    trade_date_view.add_item_str("2020-10-01");

    let mut report_name = SelectView::new().autojump().h_align(HAlign::Left);
    report_name.add_item_str("Report 1");
    report_name.add_item_str("Report 2");
    report_name.add_item_str("Report 3");
    report_name.add_item_str("Report 4");
    report_name.add_item_str("Report 5");
    report_name.add_item_str("Report 6");
    report_name.add_item_str("Report 7");
    report_name.add_item_str("Report 8");
    report_name.add_item_str("Report 9");
    report_name.add_item_str("Report 10");

    let mut report_view = SelectView::new().autojump().h_align(HAlign::Left);
    report_view.add_item_str("Report 1");

    let selections = LinearLayout::vertical()
        .child(watchlist)
        .child(trade_date_view)
        .child(report_name);

    let workspace = LinearLayout::horizontal()
        .child(selections)
        .child(report_view);

    let layout = LinearLayout::vertical()
        .child(workspace)
        .child(TextView::new("status line"));

    siv.screen_mut().add_transparent_layer(layout);

    //siv.add_global_callback('q', |s| s.quit());
}
