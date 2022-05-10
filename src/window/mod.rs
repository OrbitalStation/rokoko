//!
//! This module provides the [`Window`] type and all the things
//! connected to it.
//!
//! # Why nightly-only?
//!
//! The nightly Rust is required to provide powerful, convenient,
//! and yet highly optimized window-building model, with all the configurations being done
//! in compile-time.
//!
//! ## Let's dive into it.
//!
//! We will start that chapter from looking at this tiny example:
//! ```no_run
//! # use rokoko::prelude::*;
//! /*
//! This program prints(once) `Initialized!`
//! then creates a window with platform-preset dimensions
//! and title `rokoko window`.
//!
//! Once closed(i.e. the `close` button or Alt+F4 is pressed) it prints `Closed!`
//! and closes the window.
//! */
//! Window::new()
//!     // Called once when window is created
//!     .on_init(|_| println!("Initialized!"))
//!     // Called once when user attempts to close the window
//!     .on_close(|w| {
//!         println!("Closed!");
//!         // Close the window
//!         w.close()
//!     })
//!     // Create the window; that function never returns
//!     .create()
//!     .unwrap()
//! ```
//! Looks quite cool, isn't it? Expressive, elegant and yet simple...
//!
//! Ah, how great would it be if something like this existed, right?
//!
//! <b>So it does.</b>
//!
//! That is the opportunity to write code `rokoko` offers you.
//!
//! # How does it work under the hood
//!
//! The first idea of creating something like this was born while thinking about
//! that useless `usize` in [`Vec`], called `capacity`.
//!
//! I mean, of course capacity is needed, but not every time!
//!
//! It just wastes space, what if I want 1024 `Vec`s that are created and then not changed at all?
//!
//! That's 8Kb(`x86-64`) of wasted memory!
//!
//! So I decided to use customizable `Vec`s using type list:
//! ```no_run
//! /* Type list looks something like this */
//!
//! use std::any::TypeId;
//!
//! /// Get a type(if contained)
//! pub trait Get {
//!     fn get <T> (&self) -> Option <&T>;
//! }
//!
//! /// Terminator
//! pub struct Empty;
//!
//! impl Get for Empty {
//!     fn get <T> (&self) -> Option <&T> {
//!         None
//!     }
//! }
//!
//! /// Connector
//! pub struct With <T, N: Get> {
//!     pub data: T,
//!     pub next: N
//! }
//!
//! impl <T, N: Get> Get for With <T, N> {
//!     /// Recursion:
//!     /// If `U` is `T`(i.e. contained in that node)
//!     /// then return it.
//!     /// Delegate the task to the next node otherwise.
//!     fn get <U> (&self) -> Option <&U> {
//!         if TypeId::of::<U> == TypeId::of::<T>() {
//!             // SAFETY: T & U are same so transmuting Option <&T> to Option <&U> is safe
//!             unsafe { core::mem::transmute(Some(&self.data)) }
//!         } else {
//!             self.next.get::<U>()
//!         }
//!     }
//! }
//!
//! /// Represents capacity in a vec.
//! pub struct Capacity(usize);
//!
//! /// A vec with conditional capacity.
//! ///
//! /// # Examples
//! /// ```rust
//! /// // No capacity (size = 2 * usize)
//! /// VecWithConditionalCapacity <u32, Empty>
//! ///
//! /// // With capacity (size = 3 * usize)
//! /// VecWithConditionalCapacity <u32, With <Capacity, Empty>>
//! /// ```
//! pub struct VecWithConditionalCapacity <T, C: Get> {
//!     len: usize,
//!     ptr: *const T,
//!     component: C
//! }
//!
//! impl <T, C: Get> VecWithConditionalCapacity <T, C> {
//!     // ...
//!
//!     pub fn push(&mut self, elem: T) {
//!         if let Some(capacity) = self.component.get::<Capacity>() {
//!             // We have capacity, reserve additional elements if needed
//!         } else {
//!             // We do not have capacity, reserve exactly 1 element
//!         }
//!     }
//! }
//! ```
//!
//! As you see, there is a connector `With` and a terminator `Empty`.
//!
//! For example, `[Type1, Type2, Type3]` would look like `With <Type1, With <Type2, With <Type3, Empty>>>`.
//!
//! In this module `With` serves as a main connector of [`WindowBuilder`]'s additional properties,
//! such as info, callbacks, data, requests, etc.
//!
//! Let's return to the example from the very beginning:
//! ```no_run
//! # use rokoko::prelude::*;
//! /*
//! This program prints(once) `Initialized!`
//! and creates a window with platform-preset dimensions
//! and title `rokoko window`.
//!
//! Once closed(i.e. the `close` button or Alt+F4 is pressed) it prints `Closed!`
//! and closes the window.
//! */
//! Window::new()
//!     // Called once when window is created
//!     .on_init(|_| println!("Initialized!"))
//!     // Called once when user attempts to close the window
//!     .on_close(|w| {
//!         println!("Closed!");
//!         // Close the window
//!         w.close()
//!     })
//!     // Create the window; that function never returns
//!     .create()
//!     .unwrap()
//! ```
//!
//! The best part is that it is all completely free!
//!
//! All these `.on_init` and `.on_close` are embedded at compile-time
//! and furthermore, if, for example, `.on_init` is not specified, it will not be checked for
//! in runtime!
//!
//! The trick here is that all these `.on_init`, etc. are const functions that produce another type.
//!
//! Let's explain that example step-by-step.
//! ```no_run
//! # use rokoko::prelude::*;
//! Window::new() // Produces `WindowBuilder <Empty>`
//!     .on_init(|_| println!("Initialized!")) // Produces `WindowBuilder <With <OnEventFnContainer <OnInit, {{closure}}>, Empty>>`
//!     .on_close(|w| {
//!         println!("Closed!");
//!         w.close()
//!     }) // Produces `WindowBuilder <With <OnEventFnContainer <OnClose, {{closure}}>, With <OnEventFnContainer <OnInit, {{closure}}>, Empty>>>`
//!     .create()
//!     .unwrap()
//! ```
//! Looks scary, but every function simply adds new `With` with a new function.
//!
//! But... why not to use winit-like `WindowBuilder`? It's much more simple to implement.
//!
//! The answer to that simple and yet tricky question lies on the ground.
//!
//! What is the most important thing in `rokoko::window`? Simplicity of usage because
//! of using callbacks such as `.on_init` and etc.
//!
//! In winit-like `WindowBuilder` all the data is stored inside of the `WindowBuilder` itself,
//! so if you want callbacks - they will be checked for existence in the loop every time they need to be called.
//!
//! That is not what is called `effectiveness`, but in my model - it *is* possible.
//!
//! All the callbacks(of course with all the other data) are stored in a `type list`,
//! so their existence can be checked during compile time and thus
//! avoided in runtime, and that is what happens.
//!
//! Voila.
//!
//! The most obvious way to prove it, in my opinion, is to look at the generated assembler.
//!
//! So, let's take that example:
//! ```no_run
//! use rokoko::prelude::*;
//!
//! fn main() {
//!     Window::new()
//!         .title("Window")
//!         .on_close(Window::close)
//!         .create()
//!         .unwrap()
//! }
//! ```
//!
//! Compile it with `cargo rustc --bin rokoko --release -- --emit asm`.
//!
//! That command will generate assembler listing in `target/release/deps/rokoko-<hash>.s`.
//!
//! Open that file(be careful, ~100,000 lines) and look for a label named `main`.
//!
//! ```asm
//! .section	.text.main,"ax",@progbits
//! 	.globl	main
//! 	.p2align	4, 0x90
//! 	.type	main,@function
//! main:
//! 	.cfi_startproc
//! 	pushq	%rax
//! 	.cfi_def_cfa_offset 16
//! 	movq	%rsi, %rcx
//! 	movslq	%edi, %rdx
//! 	leaq	_ZN6rokoko4main17h799a35610e9c3a4eE(%rip), %rax
//! 	movq	%rax, (%rsp)
//! 	leaq	.L__unnamed_142(%rip), %rsi
//! 	movq	%rsp, %rdi
//! 	callq	*_ZN3std2rt19lang_start_internal17heb29ca914ff21b00E@GOTPCREL(%rip)
//! 	popq	%rcx
//! 	.cfi_def_cfa_offset 8
//! 	retq
//! .Lfunc_end621:
//! 	.size	main, .Lfunc_end621-main
//! 	.cfi_endproc
//! ```
//!
//! Apart from all the other code, that "function" calls `_ZN6rokoko4main17h799a35610e9c3a4eE`,
//! which is basically our `main` function with our code.
//! ```asm
//! _ZN6rokoko4main17h799a35610e9c3a4eE:
//! .Lfunc_begin289:
//! 	.cfi_startproc
//! 	.cfi_personality 155, DW.ref.rust_eh_personality
//! 	.cfi_lsda 27, .Lexception289
//! 	pushq	%rbx
//! 	.cfi_def_cfa_offset 16
//! 	subq	$128, %rsp
//! 	.cfi_def_cfa_offset 144
//! 	.cfi_offset %rbx, -16
//! 	callq	*_ZN6rokoko6window6Window3new17h07d10e4944267f71E@GOTPCREL(%rip)
//! 	leaq	.L__unnamed_442(%rip), %rsi
//! 	leaq	64(%rsp), %rdi
//! 	movl	$6, %edx
//! 	callq	_ZN6rokoko6window5build22WindowBuilder$LT$C$GT$6create17h4aa35c5d7be40000E
//! 	cmpl	$3, 80(%rsp)
//! 	jne	.LBB620_1
//! 	addq	$128, %rsp
//! 	.cfi_def_cfa_offset 16
//! 	popq	%rbx
//! 	.cfi_def_cfa_offset 8
//! 	retq
//! ```
//! As you can see, it calls [`rokoko::window::Window::new`] `@GOTPCREL` function.
//!
//! I honestly don't know why is it there, but that `@GOTPCREL` should definitely mean something special,
//! so let's ignore it :)
//!
//! More importantly, later [`rokoko::window::build::WindowBuilder::create`] is called,
//! and there is no configuring code inbetween,
//! so there are no configurations being done in runtime.
//!
//! Okay, I have proven that no configuration is done in runtime, but what about insides of `create`?
//!
//! Let's see another example for that:
//! ```no_run
//! use rokoko::prelude::*;
//!
//! fn main() {
//!     Window::new()
//!         .size((1000., 1000.).into())
//!         .maximized()
//!         .on_close(Window::close)
//!         .create()
//!         .unwrap()
//! }
//! ```
//!
//! As you can see, both `size` and `maximized` are specified, so the code should panic.
//!
//! The check itself is located in `create` and looks like this:
//! ```no_run
//! # use rokoko::window::build::*;
//! # let WindowBuilder(mut data) = WindowBuilder::empty();
//! if let Some(Size(size)) = data.size() {
//!     assert!(data.maximized().is_none(), "cannot have both `size` and `maximized`");
//!
//!     // ...
//! }
//! ```
//! Compile and open the assembly listing.
//!
//! The file itself reduced greatly - only about 1500 lines(previously was ~100,000):
//! that's because the compiler could find in *compile time* that the code will panic
//! anyway and thus removed window initialization(which takes LOTS of lines) completely.
//!
//! If this is not what proves that everything is done in compile time I don't know what is.
//!
//! # Drawbacks
//! Of course, this model has its drawbacks.
//!
//! The one that you can feel the most - slow compilation.
//!
//! This is because it is built over heavy metaprogramming on generics,
//!
//! and the only way to reduce the time - improve the compiler :)
//!
//! The other one that is less obvious is expressed in that question:
//!
//! What if I want to set or not to set some data/callback depending on my wish?
//!
//! I mean, look at this code:
//!
//! ```compile_fail
//! use rokoko::prelude::*;
//!
//! let mut builder = Window::new();
//!
//! let mut buf = String::new();
//! std::io::stdin().read_line(&mut buf).unwrap();
//!
//! if &*buf == "maximize\n" {
//!     builder = builder.maximize()
//! }
//!
//! builder.create().unwrap();
//! ```
//! It will not compile!
//!
//! That is because `maximize` produces _another_ type, not the one
//! `Window::new()` produces.
//!
//! Workaround is as follows(or something like this):
//! ```no_run
//! use rokoko::prelude::*;
//! use rokoko::window::build::{*, getters::*};
//! use winit::error::OsError;
//!
//! trait WindowBuildable {
//!     fn create(self) -> Result <(), OsError>;
//! }
//!
//! impl <'title, C: GetData <Title <'title>> + GetFn <OnClose> + /* lots of other traits */> WindowBuildable for WindowBuilder <C> {
//!     fn create(self) -> Result<(), OsError> {
//!        self.create()
//!     }
//! }
//!
//! let mut builder: Box <dyn WindowBuildable> = Box::new(Window::new());
//!
//! let mut buf = String::new();
//! std::io::stdin().read_line(&mut buf).unwrap();
//!
//! if &*buf == "maximize\n" {
//!     builder = Box::new(builder.maximize())
//! }
//!
//! builder.create().unwrap();
//! ```
//! Ugly, ugly, ugly, ahhh...
//!
//! The only thing that calms me down is the fact that it is not actually needed(often) to create such a window dynamically,
//! right?..
//!
//! <b>At least I hope to.</b>
//!
//! Anyway, I think that advantages of such a model are much more important than the drawbacks.
//!

///
/// Only `nightly` is supported for now.
///
#[cfg(not(nightly))]
compile_error!("Current `window` implementation requires nightly Rust.");

pub mod build;
use self::build::WindowBuilder;

pub mod data;
use self::data::{WindowData, UserEvent};

use core::ptr::NonNull;
use raw_window_handle::RawWindowHandle;

///
/// The main type of the module.
///
/// Does *not* hold any data, is just a reference to an actual window.
///
/// Why not to use `&WindowData`? References(more particularly, lifetimes)
///
/// are not easy to use when it comes to type list, so - we have what we have.
///
#[derive(Copy, Clone)]
pub struct Window(NonNull <WindowData>);

impl Window {
    /// Creates a new `WindowBuilder`, ready to be customized
    pub const fn new() -> WindowBuilder {
        WindowBuilder::empty()
    }

    ///
    /// Closes the window.
    ///
    /// Only [`WindowBuilder::on_exit`] is called after this function.
    ///
    pub fn close(self) {
       self.data().proxy.send_event(UserEvent::Close).expect("window must be opened to be closed")
    }
}

unsafe impl raw_window_handle::HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.data().winit.get().raw_window_handle()
    }
}

impl Window {
    /// Creates a new reference to `WindowData`.
    const fn from(data: &mut WindowData) -> Self {
        // SAFETY: safe because reference cannot be null
        Self(unsafe { NonNull::new_unchecked(data) })
    }

    /// Get actual window data.
    const fn data(&self) -> &WindowData {
        // SAFETY: safe because `self.0` is guaranteed to have a reference
        unsafe { &*self.0.as_ptr() }
    }
}
