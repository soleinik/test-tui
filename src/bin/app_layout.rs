use cursive::{
    align::HAlign,
    theme::{BaseColor, BorderStyle, Color, PaletteColor, Theme},
    view::Resizable,
    views::{LinearLayout, Panel, ScrollView, SelectView, TextView},
};

pub fn app_layout(siv: &mut cursive::Cursive) {
    let mut theme = Theme::default();
    theme.shadow = false;
    theme.borders = BorderStyle::Simple;
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);

    theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::White);
    theme.palette[PaletteColor::TitlePrimary] = Color::Light(BaseColor::Blue);

    theme.palette[PaletteColor::Highlight] = Color::Light(BaseColor::Blue);
    theme.palette[PaletteColor::HighlightText] = Color::Dark(BaseColor::Black);
    siv.set_theme(theme);

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
    report_name.add_item_str("Report 1, with very very long name");
    report_name.add_item_str("Report 2");
    report_name.add_item_str("Report 3");
    report_name.add_item_str("Report 4");
    report_name.add_item_str("Report 5");
    report_name.add_item_str("Report 6");
    report_name.add_item_str("Report 7");
    report_name.add_item_str("Report 8");
    report_name.add_item_str("Report 9");
    report_name.add_item_str("Report 10");

    let watch_list_scroll = ScrollView::new(watchlist.full_height()).scroll_x(true);
    let trade_date_scroll = ScrollView::new(trade_date_view.full_height());
    let report_scroll = ScrollView::new(report_name.full_height()).scroll_x(true);

    let watch_list_panel = Panel::new(watch_list_scroll)
        .title("Watchlist")
        .max_width(30);

    let trade_date_panel = Panel::new(trade_date_scroll)
        .title("Trade Date")
        .max_width(30);
    let report_panel = Panel::new(report_scroll).title("Report").max_width(30);

    let selections = LinearLayout::vertical()
        .child(watch_list_panel)
        .child(trade_date_panel)
        .child(report_panel)
        .min_width(30);

    //let selections_panel = Panel::new(selections).title("Selections").max_width(30);

    let mut report_view = SelectView::new().autojump().h_align(HAlign::Left);
    report_view.add_item_str("Report Header row");
    report_view.add_item_str("Report line 1");
    report_view.add_item_str("Report line 2");
    report_view.add_item_str("Report line 3");
    report_view.add_item_str("Report line 4");
    report_view.add_item_str("Report line 5");
    report_view.add_item_str("Report line 6");
    report_view.add_item_str("Report line 7");

    let report_scroll = ScrollView::new(report_view.full_height());

    let report_panel = Panel::new(report_scroll).title("Report").full_screen();

    let workspace = LinearLayout::horizontal()
        .child(selections)
        .child(report_panel);

    let layout = LinearLayout::vertical()
        .child(workspace.full_screen())
        .child(TextView::new("status line"));

    siv.screen_mut().add_fullscreen_layer(layout);
}
