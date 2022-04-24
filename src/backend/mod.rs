#[cfg(feature = "backend-opengl")]
pub mod opengl;

pub trait Backend {
    
}

static mut BACKEND: Option <Box <dyn Backend>> = None;
