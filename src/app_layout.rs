use cursive::{
    align::HAlign,
    theme::{BaseColor, BorderStyle, Color, PaletteColor, Theme},
    view::{Nameable, Resizable},
    views::{LinearLayout, Panel, ResizedView, ScrollView, SelectView, TextView},
    Cursive,
};

use crate::{app_ui::AppState, commands::CtlCommand};
const ID_NAME_SELECTION: &str = "id-name-selection";
const MAX_WIDTH: usize = 20;
const MIN_WIDTH: usize = 15;

const ID_INDEX_WATCHLIST: usize = 0;
const WATCHLIST_TITLE: &str = "W.lst";

const ID_INDEX_TRADEDATE: usize = 1;
const TRADEDATE_TITLE: &str = "T.Date";

const ID_INDEX_REPORTNAMES: usize = 2;
const REPORTNAMES_TITLE: &str = "Rpt.Nms";

const ID_NAME_REPORT: &str = "id-name-report";
const ID_INDEX_REPORT: usize = 1;
const REPORT_TITLE: &str = "Report";

pub const ID_NAME_MAIN: &str = "id-name-main";
pub const ID_INDEX_STATUS: usize = 1;

const ID_NAME_WATCHLIST_PANEL: &str = "id-name-watchlist-panel";
const ID_NAME_TRADEDATE_PANEL: &str = "id-name-tradedate-panel";

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

    let mut watchlist_view: SelectView<String> = SelectView::new().autojump().h_align(HAlign::Left);
    watchlist_view = watchlist_view.on_submit(watchlist_on_submit);

    let mut trade_date_view: SelectView<String> =
        SelectView::new().autojump().h_align(HAlign::Left);
    trade_date_view = trade_date_view.on_submit(tradedate_on_submit);

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

    let watch_list_panel = Panel::new(watch_list_scroll)
        .title(WATCHLIST_TITLE)
        .with_name(ID_NAME_WATCHLIST_PANEL);

    let trade_date_panel = Panel::new(trade_date_scroll)
        .title(TRADEDATE_TITLE)
        .with_name(ID_NAME_TRADEDATE_PANEL);

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

    let report_scroll = ScrollView::new(report_view.full_screen()).scroll_x(true);
    let report_panel = Panel::new(report_scroll).title(REPORT_TITLE);

    let workspace = LinearLayout::horizontal()
        .child(selections)
        .child(report_panel)
        .with_name(ID_NAME_REPORT);

    let layout = LinearLayout::vertical()
        .child(workspace)
        .child(TextView::new("status line placeholder"));

    siv.screen_mut()
        .add_fullscreen_layer(layout.with_name(ID_NAME_MAIN));
}

pub trait ManageSelectViews {
    fn set_view_watchlist(&mut self, view: SelectView);
    fn set_view_tradedate(&mut self, view: SelectView);
    fn set_view_reportnames(&mut self, view: SelectView);
    fn set_view_report(&mut self, view: LinearLayout);
}

fn watchlist_on_submit(siv: &mut Cursive, ticker: &String) {
    siv.user_data().map(|state: &mut AppState| {
        state.send(CtlCommand::SelectTicker(ticker.to_string()));
    });

    siv.call_on_name(
        ID_NAME_WATCHLIST_PANEL,
        |pnl: &mut Panel<ScrollView<ResizedView<SelectView>>>| {
            pnl.set_title(ticker);
        },
    );
}

fn tradedate_on_submit(siv: &mut Cursive, trade_date: &String) {
    siv.user_data().map(|state: &mut AppState| {
        state.send(CtlCommand::SelectTradeDate(trade_date.to_string()));
    });

    siv.call_on_name(
        ID_NAME_TRADEDATE_PANEL,
        |pnl: &mut Panel<ScrollView<ResizedView<SelectView>>>| {
            pnl.set_title(trade_date);
        },
    );
}

impl ManageSelectViews for Cursive {
    fn set_view_watchlist(&mut self, mut view: SelectView) {
        if let Some(mut layout) = self.find_name::<LinearLayout>(ID_NAME_SELECTION) {
            layout.remove_child(ID_INDEX_WATCHLIST);

            view.set_on_submit(watchlist_on_submit);

            let selected = self
                .user_data()
                .and_then(|state: &mut AppState| state.ticker());

            if let Some(ticker) = &selected {
                let found = view.iter().enumerate().find_map(|(index, item)| {
                    if item.1 == ticker {
                        Some(index)
                    } else {
                        None
                    }
                });
                if let Some(index) = found {
                    view.set_selection(index);
                }
            }

            let scroll_view = ScrollView::new(view.full_screen()).scroll_x(true);
            let panel = Panel::new(scroll_view)
                .title(selected.unwrap_or(WATCHLIST_TITLE.to_string()))
                .with_name(ID_NAME_WATCHLIST_PANEL);
            layout.insert_child(ID_INDEX_WATCHLIST, panel);
        } else {
            log::warn!("{ID_NAME_SELECTION} not found");
        }
    }

    fn set_view_tradedate(&mut self, mut view: SelectView) {
        if let Some(mut layout) = self.find_name::<LinearLayout>(ID_NAME_SELECTION) {
            layout.remove_child(ID_INDEX_TRADEDATE);

            view.set_on_submit(tradedate_on_submit);

            let selected = self
                .user_data()
                .and_then(|state: &mut AppState| state.trade_date());

            if let Some(trade_date) = &selected {
                let found = view.iter().enumerate().find_map(|(index, item)| {
                    if item.1 == trade_date {
                        Some(index)
                    } else {
                        None
                    }
                });
                if let Some(index) = found {
                    view.set_selection(index);
                }
            }

            let scroll_view = ScrollView::new(view.full_screen()).scroll_x(true);
            let panel = Panel::new(scroll_view)
                .title(selected.unwrap_or(TRADEDATE_TITLE.to_string()))
                .with_name(ID_NAME_TRADEDATE_PANEL);
            layout.insert_child(ID_INDEX_TRADEDATE, panel);
        } else {
            log::warn!("{ID_NAME_SELECTION} not found");
        }
    }

    fn set_view_reportnames(&mut self, view: SelectView) {
        if let Some(mut layout) = self.find_name::<LinearLayout>(ID_NAME_SELECTION) {
            layout.remove_child(ID_INDEX_REPORTNAMES);
            let scroll_view = ScrollView::new(view.full_screen()).scroll_x(true);
            let panel = Panel::new(scroll_view).title(REPORTNAMES_TITLE);
            layout.insert_child(ID_INDEX_REPORTNAMES, panel);
        } else {
            log::warn!("{ID_NAME_SELECTION} not found");
        }
    }

    fn set_view_report(&mut self, table: LinearLayout) {
        if let Some(mut layout) = self.find_name::<LinearLayout>(ID_NAME_REPORT) {
            layout.remove_child(ID_INDEX_REPORT);

            // let view = ScrollView::new(view.full_screen()).scroll_x(true);
            // let panel = Panel::new(view).title(REPORT_TITLE);
            layout.insert_child(ID_INDEX_REPORT, table);
        }
    }
}
