use winit::{
    event_loop::EventLoopProxy,
    window::Window as Winit
};
use core::num::NonZeroUsize;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum UserEvent {
    Close
}

/// This dirty and highly unsafe structure is needed
/// to workaround `'static` requirement by [`winit::event_loop::EventLoop::run`].
pub struct WinitRef(NonZeroUsize);

impl WinitRef {
    pub const fn new(w: &Winit) -> Self {
        // SAFETY: safe because reference cannot be null
        Self(unsafe { core::mem::transmute(w) })
    }

    pub const fn get(&self) -> &Winit {
        // SAFETY: safe because creation is only possible through `new` which
        // guarantees correctness
        unsafe { &*(self.0.get() as *const Winit) }
    }
}

pub struct WindowData {
    pub proxy: EventLoopProxy <UserEvent>,
    pub winit: WinitRef
}
