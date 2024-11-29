#[derive(Debug)]
pub enum UICommand {
    WatchList,
    TestView,
    Quit,
}

pub enum CtlCommand {
    Ready,
    SelectTicker(String),

    ///For Testing/Debuging
    Ping(UICommand),
}
