use cursive::{
    align::HAlign,
    reexports::crossbeam_channel::Select,
    theme::{Effect, Style},
    view::Resizable,
    views::{FixedLayout, LinearLayout, Panel, ScrollView, SelectView, TextView},
    Rect,
};
use grid_component::TableData;

use crate::app_layout::ManageSelectViews;

pub fn create_table(siv: &mut cursive::Cursive, data: &TableData) {
    let mut body_view: SelectView<String> = SelectView::new().autojump().h_align(HAlign::Left);

    for row in data.iter_rows() {
        let text = data.format_row(row);
        body_view.add_item_str(text);
    }

    //let body_scroll = ScrollView::new(body_view.full_screen()).scroll_x(true);

    //header
    let header_panel = create_header(data);

    //let header = LinearLayout::horizontal().child(header_panel);

    let table = LinearLayout::vertical()
        .child(header_panel)
        .child(ScrollView::new(body_view.full_screen()).scroll_y(true));

    let table = ScrollView::new(table).scroll_x(true);

    let table = LinearLayout::vertical().child(Panel::new(table));

    siv.set_view_report(table);
    //data, scrollable vertically
}

const HORIZONTAL: &str = "─";
const CROSS: &str = "┼";
const TERMCROSS: &str = "┤";

fn create_header(data: &TableData) -> LinearLayout {
    let mut layout = FixedLayout::new();
    let mut x = 0;

    let mut line = String::new();
    for (text, width) in data.column_metas.iter() {
        let content = format!("{:^width$}│", text);
        let text = TextView::new(content).style(Style::from(Effect::Bold));

        let width = width + 1;
        line += &std::iter::repeat(HORIZONTAL)
            .take(width - 1)
            .collect::<String>();
        line += CROSS;

        layout.add_child(Rect::from_size((x, 0), (width, 1)), text);
        x += width;
    }
    line.pop();
    line += TERMCROSS;

    let layout = LinearLayout::vertical()
        .child(layout)
        .child(TextView::new(line).style(Style::from(Effect::Bold)));

    layout
}
