use winit::{
    event_loop::EventLoopProxy
};

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum UserEvent {
    Close
}

pub struct WindowData {
    pub proxy: EventLoopProxy <UserEvent>
}
