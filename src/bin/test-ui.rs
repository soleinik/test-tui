use cursive::{
    align::HAlign,
    event,
    theme::{BaseColor, BorderStyle, Color, PaletteColor, Theme},
    view::{Nameable, Resizable},
    views::{LinearLayout, Panel, ResizedView, ScrollView, SelectView, TextView},
    Cursive, View,
};
const ID_NAME_SELECTION: &str = "id-name-selection";
const MAX_WIDTH: usize = 40;
const MIN_WIDTH: usize = 20;

const ID_INDEX_WATCHLIST: usize = 0;
const WATCHLIST_TITLE: &str = "W.lst";

const ID_INDEX_TRADEDATE: usize = 1;
const TRADEDATE_TITLE: &str = "T.Date";

const ID_INDEX_REPORTNAMES: usize = 2;
const REPORTNAMES_TITLE: &str = "Rpt.Nms";

const ID_NAME_REPORT: &str = "id-name-report";
const ID_INDEX_REPORT: usize = 1;
const REPORT_TITLE: &str = "Report";

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

    let mut watchlist_view = SelectView::new().autojump().h_align(HAlign::Left);
    watchlist_view.add_item_str("NVDA");
    watchlist_view.add_item_str("FBTC");
    watchlist_view.add_item_str("WULF");
    watchlist_view.add_item_str("MARA");
    watchlist_view.add_item_str("HIMS");
    watchlist_view.add_item_str("IREN");
    watchlist_view.add_item_str("HUT");
    watchlist_view.add_item_str("^SPX");
    watchlist_view
        .add_item_str("1234567890123456789012345678901234567890123456789012345678901234567890");

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
    trade_date_view.add_item_str("12456789012345678901234567890");

    let mut report_name = SelectView::new().autojump().h_align(HAlign::Left);
    report_name.add_item_str("Report 1, with very very long name");
    report_name.add_item_str("12456789012345678901234567890");
    report_name.add_item_str("Report 1");
    report_name.add_item_str("Report 2");
    report_name.add_item_str("Report 3");
    report_name.add_item_str("Report 4");
    report_name.add_item_str("Report 5");
    report_name.add_item_str("Report 6");
    report_name.add_item_str("Report 7");
    report_name.add_item_str("Report 8");

    let watch_list_scroll = ScrollView::new(watchlist_view.full_screen()).scroll_x(true);
    let trade_date_scroll = ScrollView::new(trade_date_view.full_screen()).scroll_x(true);
    let report_name_scroll = ScrollView::new(report_name.full_screen()).scroll_x(true);

    let watch_list_panel = Panel::new(watch_list_scroll).title(WATCHLIST_TITLE);
    let trade_date_panel = Panel::new(trade_date_scroll).title(TRADEDATE_TITLE);
    let report_name_panel = Panel::new(report_name_scroll).title(REPORTNAMES_TITLE);

    let selections = LinearLayout::vertical()
        .child(watch_list_panel)
        .child(trade_date_panel)
        .child(report_name_panel)
        .with_name(ID_NAME_SELECTION);

    let selections = ResizedView::with_max_width(MAX_WIDTH, selections).min_width(MIN_WIDTH);

    let mut report_view = SelectView::new().autojump().h_align(HAlign::Left);

    report_view.add_item_str("Report Header row");
    report_view.add_item_str("Report line 1");
    report_view.add_item_str("Report line 2");
    report_view.add_item_str("Report line 3");
    report_view.add_item_str("Report line 4");
    report_view.add_item_str("Report line 5");
    report_view.add_item_str("Report line 6");
    report_view.add_item_str("Report line 7");

    //let report_scroll = ScrollView::new(report_view.full_height()).scroll_x(true);
    let report_scroll = ScrollView::new(report_view.full_screen()).scroll_x(true);
    let report_panel = Panel::new(report_scroll).title(REPORT_TITLE);

    let workspace = LinearLayout::horizontal()
        .child(selections)
        .child(report_panel)
        .with_name(ID_NAME_REPORT);

    let layout = LinearLayout::vertical()
        .child(workspace)
        .child(TextView::new("status line"));

    siv.screen_mut().add_fullscreen_layer(layout);

    siv.find_name::<LinearLayout>(ID_NAME_SELECTION).unwrap();
}

pub trait ManageSelectViews {
    fn set_view_watchlist<T: View>(&mut self, view: T);
    fn set_view_tradedate<T: View>(&mut self, view: T);
    fn set_view_reportnames<T: View>(&mut self, view: T);
    fn set_view_report<T: View>(&mut self, view: T);
}

impl ManageSelectViews for Cursive {
    fn set_view_watchlist<T: View>(&mut self, view: T) {
        if let Some(mut layout) = self.find_name::<LinearLayout>(ID_NAME_SELECTION) {
            layout.remove_child(ID_INDEX_WATCHLIST);
            let view = ScrollView::new(view.full_screen()).scroll_x(true);
            let panel = Panel::new(view).title(WATCHLIST_TITLE);
            layout.insert_child(ID_INDEX_WATCHLIST, panel);
        } else {
            log::warn!("{ID_NAME_SELECTION} not found");
        }
    }

    fn set_view_tradedate<T: View>(&mut self, view: T) {
        if let Some(mut layout) = self.find_name::<LinearLayout>(ID_NAME_SELECTION) {
            layout.remove_child(ID_INDEX_TRADEDATE);
            let view = ScrollView::new(view.full_screen()).scroll_x(true);
            let panel = Panel::new(view).title(TRADEDATE_TITLE);
            layout.insert_child(ID_INDEX_TRADEDATE, panel);
        } else {
            log::warn!("{ID_NAME_SELECTION} not found");
        }
    }

    fn set_view_reportnames<T: View>(&mut self, view: T) {
        if let Some(mut layout) = self.find_name::<LinearLayout>(ID_NAME_SELECTION) {
            layout.remove_child(ID_INDEX_REPORTNAMES);
            let view = ScrollView::new(view.full_screen()).scroll_x(true);
            let panel = Panel::new(view).title(REPORTNAMES_TITLE);
            layout.insert_child(ID_INDEX_REPORTNAMES, panel);
        } else {
            log::warn!("{ID_NAME_SELECTION} not found");
        }
    }

    fn set_view_report<T: View>(&mut self, view: T) {
        if let Some(mut layout) = self.find_name::<LinearLayout>(ID_NAME_REPORT) {
            layout.remove_child(ID_INDEX_REPORT);

            let view = ScrollView::new(view.full_screen()).scroll_x(true);
            let panel = Panel::new(view).title(REPORT_TITLE);
            layout.insert_child(ID_INDEX_REPORT, panel);
        }
    }
}

fn main() {
    cursive::logger::set_internal_filter_level(log::LevelFilter::Warn);
    cursive::logger::init();

    let mut siv = cursive::default();

    siv.add_global_callback('~', cursive::Cursive::toggle_debug_console);

    app_layout(&mut siv);

    // let zz = siv
    //     .find_name::<Panel<ScrollView<ResizedView<SelectView>>>>(ID_NAME_REPORT)
    //     .unwrap();~

    // log::warn!("found it in Runnable!");

    siv.set_global_callback(event::Key::F5, |siv| {
        let mut view = SelectView::new().autojump().h_align(HAlign::Left);

        view.add_item_str("Fresh WL 1");
        view.add_item_str("Fresh WL 2");
        view.add_item_str("1234567890123456789012345678901234567890123456789012345678901234567890");
        view.add_item_str("Fresh line 4");
        view.add_item_str("Fresh line 5");

        siv.set_view_watchlist(view);
    });
    siv.set_global_callback(event::Key::F6, |siv| {
        let mut view = SelectView::new().autojump().h_align(HAlign::Left);

        view.add_item_str("Fresh Report 1");
        view.add_item_str("Fresh line 2");
        view.add_item_str("Fresh line 3");
        view.add_item_str("Fresh line 4");
        view.add_item_str("Fresh line 5");

        siv.set_view_report(view);
    });

    siv.set_global_callback(event::Key::F7, |siv| {
        let mut view = SelectView::new().autojump().h_align(HAlign::Left);

        view.add_item_str("Fresh TD 1");
        view.add_item_str("Fresh line 2");
        view.add_item_str("Fresh line 3");
        view.add_item_str("Fresh line 4");
        view.add_item_str("Fresh line 5");

        siv.set_view_tradedate(view);
    });

    siv.set_global_callback(event::Key::F8, |siv| {
        let mut view = SelectView::new().autojump().h_align(HAlign::Left);

        view.add_item_str("Fresh RN 1");
        view.add_item_str("Fresh line 2");
        view.add_item_str("Fresh line 3");
        view.add_item_str("Fresh line 4");
        view.add_item_str("Fresh line 5");

        siv.set_view_reportnames(view);
    });

    siv.run();
}
