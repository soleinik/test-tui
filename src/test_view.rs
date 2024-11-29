// use cursive::{
//     align::HAlign,
//     event::EventResult,
//     view::CannotFocus,
//     views::{Panel, TextView},
//     Cursive, Printer, View,
// };

use cursive::Cursive;

pub fn test_view(siv: &mut Cursive) {
    //     // Create the main content
    //     let content = TextView::new("Hello, Cursive!").h_align(HAlign::Center);
    //     // Create the status line
    //     let status_line = TextView::new("Status: Ready").h_align(HAlign::Left);
    //     // Create the custom view with the status line
    //     let status_line_view = StatusLineView::new(Panel::new(content), Panel::new(status_line));

    //     // Add the custom view as a fullscreen layer
    //     siv.add_fullscreen_layer(status_line_view);
}

// struct StatusLineView<V> {
//     inner: V,
//     status: TextView,
// }

// impl<V: View> View for StatusLineView<V> {
//     fn draw(&self, printer: &Printer) {
//         let size = printer.size;
//         self.inner
//             .draw(&printer.offset((0, 0)).crop(size.saturating_sub((0, 1))));
//         self.status
//             .draw(&printer.offset((0, size.y - 1)).fixed_size((size.x, 1)));
//     }

//     fn required_size(&mut self, req: cursive::Vec2) -> cursive::Vec2 {
//         let inner_size = self.inner.required_size(req);
//         let status_size = self.status.required_size(req);
//         inner_size + (0, status_size.y)
//     }

//     fn layout(&mut self, size: cursive::Vec2) {
//         self.inner.layout(size.saturating_sub((0, 1)));
//         self.status.layout((size.x, 1).into());
//     }

//     fn needs_relayout(&self) -> bool {
//         self.inner.needs_relayout() || self.status.needs_relayout()
//     }

//     fn take_focus(
//         &mut self,
//         source: cursive::direction::Direction,
//     ) -> Result<EventResult, CannotFocus> {
//         self.inner.take_focus(source)
//     }

//     fn on_event(&mut self, event: cursive::event::Event) -> cursive::event::EventResult {
//         self.inner.on_event(event)
//     }
// }

// impl<V> StatusLineView<V> {
//     fn new(inner: V, status: TextView) -> Self {
//         Self { inner, status }
//     }
// }
