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

pub mod config;
use crate::config;

use crate::{
    nightly,
    math::vec::vec2
};
use super::{
    Window, UserEvent,
    data::{WindowData, WinitRef}
};
use winit::{
    error::OsError,
    window::WindowBuilder as WinitWindowBuilder,
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent},
    dpi::{PhysicalSize, LogicalSize, Size as WinitSize}
};

///
/// Type used to provide a convenient interface to window creation.
///
/// All the explanations can be found in `window` module.
///
pub struct WindowBuilder <C = Empty> (C);

config! {
    'data:
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
        Title <('title)> (title: &'title str), TitleTrait,

        ///
        /// ## Signature
        /// `.size(vec2)` -> specifies dimensions of the window.
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
        ///     .size((1000., 1000.).into());
        /// ```
        ///
        Size(size: vec2), SizeTrait,

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
        Maximized(maximized), MaximizedTrait,

        ///
        /// ## Signature
        /// `.size_is_logical()` -> specifies that specified [`WindowBuilder::size`] is in [`winit::dpi::LogicalSize`]
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
        ///     .size((1000., 1000.).into())
        ///     .size_is_logical();
        /// ```
        ///
        SizeIsLogical(size_is_logical), SizeIsLogicalTrait

    'events:
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
        on_close(Window), OnClose, OnCloseTrait,

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
        on_init(Window), OnInit, OnInitTrait,

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
        on_exit(Window), OnExit, OnExitTrait

    'impl:
        impl <C: Config> WindowBuilder <C> {
            pub fn create(self) -> Result <(), OsError> {
                let Self(mut data) = self;

                let mut wb = WinitWindowBuilder::new().with_title(if let Some(Title(title)) = data.title() {
                    title
                } else {
                    "rokoko window"
                });

                if let Some(Size(size)) = data.size() {
                    assert!(data.maximized().is_none(), "cannot have both `size` and `maximized`");

                    wb = wb.with_inner_size(if let Some(SizeIsLogical) = data.size_is_logical() {
                        WinitSize::Logical(LogicalSize::from(*size).cast())
                    } else {
                        WinitSize::Physical(PhysicalSize {
                            width: size[0] as _,
                            height: size[1] as _
                        })
                    })
                } else if let Some(_) = data.size_is_logical() {
                    panic!("cannot have specification that `size` is logical without specification of the `size` itself")
                }

                if data.maximized().is_some() {
                    wb = wb.with_maximized(true)
                }

                let event_loop = EventLoop::with_user_event();

                let w = wb.build(&event_loop)?;

                let mut window = WindowData {
                    proxy: event_loop.create_proxy(),
                    winit: WinitRef::new(&w)
                };

                let window_ref = Window::from(&mut window);

                if let Some(cb) = data.on_init() {
                    cb(window_ref)
                }

                event_loop.run(move |event, _, control_flow| {
                    if *control_flow == ControlFlow::Exit {
                        return
                    }
                    *control_flow = ControlFlow::Wait;

                    match event {
                        Event::UserEvent(event) => match event {
                            UserEvent::Close => {
                                if let Some(cb) = data.on_exit() {
                                    cb(window_ref)
                                }
                                *control_flow = ControlFlow::Exit
                            }
                        },
                        Event::WindowEvent {
                            event: WindowEvent::CloseRequested,
                            ..
                        } => if let Some(cb) = data.on_close() {
                            cb(window_ref)
                        } else {
                            window_ref.close()
                        },
                        _ => ()
                    }
                })
            }
        }
}

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
/// Does not call `from`'s `Drop`(if exists).
///
/// The latter allows to cast [`WindowBuilder`] into its generic `C`.
///
#[doc(hidden)]
pub const unsafe fn transmute <F, T> (from: F) -> T {
    core::ptr::read(&core::mem::ManuallyDrop::new(from) as *const _ as *const T)
}
