#[derive(Debug)]
pub enum CtlCommand {
    SelectTicker(String),
    SelectTradeDate(String),
    Quit,
}
