#[derive(Debug)]
pub enum UICommand {
    WatchList,
    TestView,
    Quit,
}

pub enum CtlCommand {
    Ready,
    WatchList,
    SelectTicker(String),

    ///For Testing/Debuging
    Ping(UICommand),
}
