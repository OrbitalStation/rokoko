use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "math")] {
        pub use math::vec::vec;
        pub use math::vec::alias::*;
    }
}

cfg_if! {
    if #[cfg(feature = "window")] {
        pub use window::Window;
    }
}
