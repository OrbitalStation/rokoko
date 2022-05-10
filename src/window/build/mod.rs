//!
//! This module provides `WindowBuilder` type and all the associated
//! callbacks and etc.
//!

pub mod fn_container;
use self::fn_container::{FnContainer, NotFnContainer, OnEventFnContainer, Callback};

pub mod not_matching;
use self::not_matching::NotMatching;

pub mod equality;
use self::equality::{Equality, NotEq};

pub mod type_list;
use self::type_list::{With, Empty};

pub mod getters;
use self::getters::{GetFn, GetData};

use crate::math::vec::vec2;
use super::{
    Window, UserEvent,
    data::{WindowData, WinitRef}
};
use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent},
    dpi::{PhysicalSize, LogicalSize}
};

///
/// Type used to provide a convenient interface to window creation.
///
/// All the explanations can be found in `window` module.
///
pub struct WindowBuilder <C = Empty> (C);

rokoko_macro::window_builder_data! {
    ///
    /// ## Signature
    /// `.title(&str)` -> specifies a title to the window.
    ///
    /// ## Default
    /// Default is `"rokoko window"`.
    ///
    /// ## Example
    /// ```
    /// # use rokoko::window::Window;
    ///
    /// Window::new()
    ///     .title("Some custom title");
    /// ```
    ///
    #[default = "rokoko window"]
    #[usage = .with_title(title)]
    title: &str,

    ///
    /// ## Signature
    /// `.size(impl Into <vec2>)` -> specifies dimensions of the window.
    ///
    /// ## Default
    /// Default is some platform-dependent preset dimensions.
    ///
    /// # Compatibility
    /// Not compatible with the [`WindowBuilder::maximized`]
    ///
    /// ## Note
    /// The default type of specified `size` is [`winit::dpi::PhysicalSize`].
    ///
    /// You can change default [`winit::dpi::PhysicalSize`] to [`winit::dpi::LogicalSize`]
    /// by specifying [`WindowBuilder::size_is_logical`].
    ///
    /// See [`winit::dpi`] module documentation for more information.
    ///
    /// ## Example
    /// ```
    /// # use rokoko::window::Window;
    ///
    /// Window::new()
    ///     .size((1000., 1000.));
    /// ```
    ///
    #[conflict = maximized]
    #[usage = .with_inner_size(if data.size_is_logical().is_some() {
        winit::dpi::Size::Logical(LogicalSize::from(size).cast())
    } else {
        winit::dpi::Size::Physical(PhysicalSize {
            width: size[0] as _,
            height: size[1] as _
        })
    })]
    size: vec2,

    ///
    /// ## Signature
    /// `.maximized()` -> specifies that window should have the maximum possible size.
    ///
    /// ## Compatibility
    /// Not compatible with the [`WindowBuilder::size`]
    ///
    /// ## Example
    /// ```
    /// # use rokoko::window::Window;
    ///
    /// Window::new()
    ///     .maximized();
    /// ```
    ///
    #[conflict = size]
    #[usage = .with_maximized(true)]
    maximized,

    ///
    /// ## Signature
    /// `.size_is_logical()` -> specifies that given [`WindowBuilder::size`] is in [`winit::dpi::LogicalSize`]
    /// instead of [`winit::dpi::PhysicalSize`]
    ///
    /// ## Note
    /// Should always be used in pair with [`WindowBuilder::size`]
    ///
    /// ## Example
    /// ```
    /// # use rokoko::window::Window;
    ///
    /// Window::new()
    ///     .size((1000., 1000.))
    ///     .size_is_logical();
    /// ```
    ///
    #[require = size]
    size_is_logical
}

rokoko_macro::window_builder_events! {
    ///
    /// ## Signature
    /// `.on_close <F: FnMut(Window)> (F)` -> sets a callback that would be called when user attempts to close the window,
    /// *not* when it is actually closed.
    ///
    /// ## Default
    /// Default behaviour is that if `.on_close` is not specified then window will be simply closed
    ///
    /// ## Note
    /// If you specify `.on_close` multiple times only the very last one will be used
    ///
    /// ## Note
    /// See also [`WindowBuilder::on_exit`]
    ///
    /// ## Examples
    /// With logging:
    /// ```
    /// # use rokoko::window::Window;
    /// Window::new()
    ///     .on_close(|w| {
    ///         println!("Mama, I'm closing!");
    ///         w.close()
    ///     });
    /// ```
    /// With a counter:
    /// ```
    /// # use rokoko::window::Window;
    /// let mut counter = 5;
    ///
    /// Window::new()
    ///     .on_close(move |w| {
    ///         counter -= 1;
    ///         if counter == 0 {
    ///             println!("Closing!");
    ///             w.close()
    ///         } else {
    ///             println!("{counter} attempts to close the window remains!")
    ///         }
    ///     });
    /// ```
    /// Default:
    /// ```
    /// # use rokoko::window::Window;
    /// Window::new()
    ///     .on_close(Window::close);
    /// ```
    /// Without closing:
    /// ```
    /// # use rokoko::window::Window;
    /// Window::new()
    ///     .on_close(|_| println!("Haha, you cannot close me!"));
    /// ```
    ///
    #[on = Event::WindowEvent { event: WindowEvent::CloseRequested, .. }]
    #[default = window.close()]
    on_close(window: Window),

    ///
    /// ## Signature
    /// `.on_init <F: FnMut(Window)> (F)` -> sets a callback that will be called when the window is created.
    ///
    /// ## Note
    /// If you specify `.on_init` multiple times only the very last one will be used
    ///
    /// ## Examples
    /// With logging:
    /// ```
    /// # use rokoko::window::Window;
    /// Window::new()
    ///     .on_init(|_| println!("Initialized!"));
    /// ```
    /// Closing the window immediately:
    /// ```
    /// # use rokoko::window::Window;
    /// Window::new()
    ///     .on_init(|w| {
    ///         println!("Initialized.. Oops, sorry, already closing!");
    ///         w.close()
    ///     });
    /// ```
    ///
    #[unique = "init"]
    on_init(window: Window),

    ///
    /// ## Signature
    /// `.on_exit <F: FnMut(Window)> (F)` -> sets a callback that will be called when the `Window::close` function
    /// is called.
    ///
    /// ## Note
    /// No other callback is called after that one, so it is useful to work as a destructor
    ///
    /// ## Note
    /// If you specify `.on_exit` multiple times only the very last one will be used
    ///
    /// ## Note
    /// See also [`WindowBuilder::on_close`]
    ///
    /// ## Examples
    /// ```
    /// # use rokoko::window::Window;
    /// struct DropMe;
    ///
    /// impl Drop for DropMe {
    ///     fn drop(&mut self) {
    ///         println!("Dropping!")
    ///     }
    /// }
    ///
    /// let to_be_dropped = DropMe;
    ///
    /// Window::new()
    ///     .on_exit(move |_| {
    ///         // SAFETY: nothing else can use `to_be_dropped` after that callback
    ///         // so dropping it here is safe
    ///         drop(unsafe { core::ptr::read(&to_be_dropped) })
    ///     });
    /// ```
    ///
    #[on = Event::UserEvent(UserEvent::Close)]
    on_exit(window: Window)
}

rokoko_macro::window_builder_create!();

impl WindowBuilder {
    ///
    /// Creates an empty [`WindowBuilder`].
    ///
    pub const fn empty() -> Self {
        Self(Empty)
    }
}

impl <C> WindowBuilder <C> {
    const fn on_event <ID: Callback, F: FnMut <ID::Args, Output = ID::Output>> (self, cb: F) -> WindowBuilder <With <OnEventFnContainer <ID, F>, C>> {
        WindowBuilder(With {
            data: FnContainer::new(cb),
            next: self.to_inner()
        })
    }

    ///
    /// Transforms the [`WindowBuilder`] into `C`.
    ///
    const fn to_inner(self) -> C {
        // SAFETY: safe because [`WindowBuilder`] does contain the only field -> `C`,
        // so its memory layout is just the same as of `C`, and because [`WindowBuilder`]
        // does not have a [`Drop`] implemented(of course), it doesn't need to be dropped.
        unsafe { transmute(self) }
    }
}

///
/// Works as [`core::mem::transmute`],
/// but does not forbid types of different sizes/containing
/// generics.
///
/// Does not call `from`'s `Drop`(if it exists).
///
/// The latter allows to conveniently cast [`WindowBuilder`] into its generic `C`.
///
#[doc(hidden)]
pub const unsafe fn transmute <F, T> (from: F) -> T {
    core::ptr::read(&core::mem::ManuallyDrop::new(from) as *const _ as *const T)
}
