use tokio::sync::mpsc::UnboundedSender;

use crate::applet::{Applet, AppletInfo, AppletPosition, AppletRequest};

#[derive(Debug, Clone)]
pub enum RuntimeRequest {
    Launch {
        info: AppletInfo,
        order: u32,
        position: AppletPosition,
    },
}

#[derive(Debug, Clone)]
pub enum RuntimeMessage {
    Ready(UnboundedSender<RuntimeRequest>),
    New(Applet),
    Request(AppletRequest, u32),
    Shutdown(u32),
}
