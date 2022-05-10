//
// This module provides macros for the `WindowBuilder`
//

///
/// Describes data to be added to `WindowBuilder`.
///
/// # Side effects
/// Affects statics in `wb_statics`
///
/// # Example
///
/// ```norun
/// window_builder_data!(title: &str, size: vec2, maximized);
/// ```
///
#[proc_macro]
#[doc(hidden)]
pub fn window_builder_data(input: TokenStream) -> TokenStream {
    use syn::{
        Ident, Attribute, Type, Token,
        punctuated::Punctuated,
        parse::{Parse, ParseStream},
        __private::ToTokens
    };

    /// A field to be added to `WindowBuilder`
    struct Data {
        attrs: Vec <Attribute>,
        ident: String,
        ty: Option <Box <Type>>
    }

    impl Parse for Data {
        fn parse(input: ParseStream) -> syn::Result <Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let ident = input.parse::<Ident>()?.to_string();
            let ty = if input.peek(Token![:]) {
                let _ = input.parse::<Token![:]>();
                Some(input.parse()?)
            } else {
                None
            };
            Ok(Data {
                attrs,
                ident,
                ty
            })
        }
    }

    /// Wrapper to bind [`Parse`] to [`Punctuated`]
    struct Fields(pub Punctuated <Data, Token![,]>);

    impl Parse for Fields {
        fn parse(input: ParseStream) -> syn::Result <Self> {
            Ok(Self(Punctuated::parse_terminated(input)?))
        }
    }

    let Fields(fields) = syn::parse_macro_input!(input);

    let mut result = String::new();

    let mut lifetimes_num = 0;

    for field in fields {
        let Data {
            mut attrs,
            ident,
            ty
        } = field;

        wb_statics::Data::add(ident.clone(), ty.is_none(), &mut attrs);

        let (inner, braced_lifetimes, lifetimes) = if ty.is_some() {
            let mut lifetimes = String::new();
            let mut inner_ty = ty.to_token_stream().to_string();
            let mut start = 0;
            for i in lifetimes_num..inner_ty.chars().filter(|c| *c == '&').count() + lifetimes_num {
                let pos = inner_ty[start..].find('&').unwrap() + 1;
                let inserted = format!("'l{i}");
                inner_ty.insert_str(pos, &inserted);
                start = pos + inserted.len();
                lifetimes.push_str(&format!("'l{i},"));
                lifetimes_num += 1
            }

            let braced_lifetimes = if lifetimes.is_empty() {
                String::new()
            } else {
                wb_statics::add_lifetimes(lifetimes.clone());
                format!("<{lifetimes}>")
            };

            (
                format!("({inner_ty})"),
                braced_lifetimes,
                lifetimes
            )
        } else {(
            String::new(),
            String::new(),
            String::new()
        )};

        let data_ty = tools::snake_to_upper_case(&ident);

        let data_trait = data_ty.clone() + "Trait";

        wb_statics::add_trait(data_trait.clone() + &braced_lifetimes);

        let attrs = attrs
            .into_iter()
            .map(|a| a.to_token_stream().to_string())
            .collect::<Vec<_>>()
            .join("\n");

        result.push_str(&format!("
pub struct {data_ty} {braced_lifetimes} {inner};

pub trait {data_trait} {braced_lifetimes} {{
    fn {ident}(&self) -> Option <&{data_ty} {braced_lifetimes}>;
}}

impl <{lifetimes} C: ~const GetData <{data_ty} {braced_lifetimes}>> const {data_trait} {braced_lifetimes} for C {{
    #[inline(always)]
    fn {ident}(&self) -> Option <&{data_ty} {braced_lifetimes}> {{
        self.get()
    }}
}}
        "));

        result.push_str(&if ty.is_some() {
            format!("
impl <C> WindowBuilder <C> {{
    {attrs}
    pub const fn {ident} <{lifetimes} T: ~const Into <{inner}>> (self, x: T)
        -> WindowBuilder <With <{data_ty} {braced_lifetimes}, C>> {{
        WindowBuilder(With {{
            data: {data_ty}(x.into()),
            next: self.to_inner()
        }})
    }}
}}
            ")
        } else {
            format!("
impl <C> WindowBuilder <C> {{
    {attrs}
    pub const fn {ident}(self)
        -> WindowBuilder <With <{data_ty}, C>> {{
        WindowBuilder(With {{
            data: {data_ty},
            next: self.to_inner()
        }})
    }}
}}
            ")
        })
    }

    result.parse().unwrap()
}

///
/// Describes events to be added to `WindowBuilder`.
///
/// # Side effects
/// Affects statics in `wb_statics`
///
/// # Example
///
/// ```norun
/// window_builder_events!(on_close(Window), on_some_event(Window, a: u8, b: f64));
/// ```
///
#[proc_macro]
#[doc(hidden)]
pub fn window_builder_events(input: TokenStream) -> TokenStream {
    use syn::{
        Ident, Type, Token, Attribute, ReturnType,
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        __private::ToTokens
    };

    /// A name with bound type
    struct Variable {
        name: String,
        ty: Box <Type>
    }

    impl Parse for Variable {
        fn parse(input: ParseStream) -> syn::Result <Self> {
            let name = input.parse::<Ident>()?.to_string();
            let _: Token![:] = input.parse()?;
            let ty: Box <Type> = Box::new(input.parse()?);
            Ok(Self {
                name,
                ty
            })
        }
    }

    /// A callback to be added to `WindowBuilder`
    struct Callback {
        attrs: Vec <Attribute>,
        ident: String,
        args: Punctuated <Variable, Token![,]>,
        ret: ReturnType
    }

    impl Parse for Callback {
        fn parse(input: ParseStream) -> syn::Result <Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let ident = input.parse::<Ident>()?.to_string();

            let content;
            syn::parenthesized!(content in input);
            let args = Punctuated::parse_terminated(&content)?;

            let ret: ReturnType = input.parse()?;

            Ok(Self {
                attrs,
                ident,
                args,
                ret
            })
        }
    }

    /// Wrapper to bind [`Parse`] to [`Punctuated`]
    struct Callbacks(pub Punctuated <Callback, Token![,]>);

    impl Parse for Callbacks {
        fn parse(input: ParseStream) -> syn::Result <Self> {
            Ok(Self(Punctuated::parse_terminated(input)?))
        }
    }

    let Callbacks(cbs) = syn::parse_macro_input!(input);

    let mut result = String::new();

    for cb in cbs {
        let Callback {
            mut attrs,
            ident,
            args,
            ret
        } = cb;

        wb_statics::Callback::add(ident.clone(), args.iter().map(|p| p.name.clone()).collect::<Vec <_>>().join(","), &mut attrs);

        let cb_ty = tools::snake_to_upper_case(&ident);

        let cb_trait = cb_ty.clone() + "Trait";

        wb_statics::add_trait(cb_trait.clone());

        let ret = match ret {
            ReturnType::Default => String::from("()"),
            ReturnType::Type(_, ty) => ty.to_token_stream().to_string()
        };

        let attrs = attrs
            .into_iter()
            .map(|a| a.to_token_stream().to_string())
            .collect::<Vec<_>>()
            .join("\n");

        let args = args
            .into_iter()
            .map(|Variable { ty, .. }| ty.to_token_stream().to_string())
            .collect::<Vec<_>>()
            .join(",");

        result.push_str(&format!("
pub struct {cb_ty};

pub trait {cb_trait}: GetFn <{cb_ty}> {{
    fn {ident}(&mut self) -> Option <&mut Self::Type>;
}}

impl <C: ~const GetFn <{cb_ty}>> const {cb_trait} for C {{
    #[inline(always)]
    fn {ident}(&mut self) -> Option <&mut Self::Type> {{
        self.get()
    }}
}}

impl Callback for {cb_ty} {{
    type Output = {ret};
    type Args = ({args},);
}}

impl <C> WindowBuilder <C> {{
    {attrs}
    pub const fn {ident} <F: FnMut <<{cb_ty} as Callback>::Args, Output = <{cb_ty} as Callback>::Output>> (self, cb: F)
        -> WindowBuilder <With <OnEventFnContainer <{cb_ty}, F>, C>> {{
        self.on_event::<{cb_ty}, F>(cb)
    }}
}}
        "))
    }

    result.parse().unwrap()
}

///
/// Constructs all data & events into a single `WindowBuilder::create` method.
///
/// # Side effects
/// Affects statics in `wb_statics`
///
#[proc_macro]
pub fn window_builder_create(_: TokenStream) -> TokenStream {
    ///
    /// A pair of usizes.
    ///
    /// The main feature of `Pair` is that `Pair(a, b) == Pair(b, a)`,
    /// i.e. order of members does not matter
    ///
    #[derive(Debug, Eq)]
    struct Pair {
        a: usize,
        b: usize
    }

    impl Pair {
        pub const fn new(a: usize, b: usize) -> Self {
            Self { a, b }
        }
    }

    impl PartialEq for Pair {
        fn eq(&self, other: &Self) -> bool {
            (self.a == other.a && self.b == other.b)
            || (self.a == other.b && self.b == other.a)
        }
    }

    /// Represents a `#[conflict]` between data
    #[derive(Debug)]
    struct Conflict {
        pair: Pair,
        /// Trick: if `met` == 2 then both in pair respect it
        met: u8
    }

    let lifetimes = wb_statics::lifetimes();
    let traits = wb_statics::traits();

    let mut data = String::new();
    let full = wb_statics::Data::get();
    let mut conflicts_to_be_checked = Vec::new();
    let mut conflicts = String::new();
    let mut requirements = String::new();

    for (idx, one) in full.iter().enumerate() {
        let lower = &one.lower;

        // Usage
        let usage = &one.usage;

        if !usage.is_empty() {
            let (wrapper, deref) = if one.short {
                (String::from("_"), String::new())
            } else {
                let upper = tools::snake_to_upper_case(&*lower);
                (format!("{upper}({lower})"), format!("let {lower} = *{lower};"))
            };

            let else_branch = if one.default.is_empty() {
                String::new()
            } else {
                let default = &one.default;
                format!("
else {{
    let {lower} = {default};
    builder = builder{usage}
}}
                ")
            };

            data.push_str(&format!("
if let Some({wrapper}) = data.{lower}() {{
    {deref}
    builder = builder{usage}
}} {else_branch}
            "))
        }

        // Requirements
        for require in &one.require {
            requirements.push_str(&format!(r#"assert!(data.{lower}().is_none() || data.{require}().is_some(), "{lower} requires {require}, which is not specified");"#));
        }

        // Conflicts
        for conflict in &one.conflict {
            let pair = Pair::new(idx, full
                .iter()
                .enumerate()
                .find(|(_, p)| p.lower == *conflict)
                .expect("no such data")
                .0);
            if let Some(c) = conflicts_to_be_checked.iter_mut().find(|p: &&mut Conflict| p.pair == pair) {
                c.met += 1
            } else {
                conflicts.push_str(&format!(r#"assert!(data.{conflict}().is_none() || data.{lower}().is_none(), "cannot have both `{conflict}` and `{lower}`");"#));
                conflicts_to_be_checked.push(Conflict {
                    pair,
                    met: 1
                })
            }
        }
    }

    for conflict in conflicts_to_be_checked {
        if conflict.met != 2 {
            panic!("only one of `{}`, `{}` specifies that they conflict", full[conflict.pair.a].lower, full[conflict.pair.b].lower)
        }
    }

    let mut events = String::new();
    let full = wb_statics::Callback::get();
    let mut unique_init = String::new();

    for one in &full {
        let lower = &one.lower;
        let args = &one.args;

        if one.unique == "init" {
            unique_init = format!("
if let Some(cb) = data.{lower}() {{
    cb({args})
}}
            ")
        } else if !one.unique.is_empty() {
            panic!("unknown value for #[unique] = {}", one.unique)
        } else {
            let on = &one.on;
            let else_branch = if one.default.is_empty() {
                String::new()
            } else {
                let default = &one.default;
                format!("
else {{
    {default}
}}
                ")
            };
            let call = format!("
if let Some(cb) = data.{lower}() {{
    cb({args})
}} {else_branch}
            ");
            let branch = if on.find("UserEvent :: Close").is_some() {
                format!("{{
{call}
*cf = ControlFlow::Exit
                }}")
            } else {
                call
            };
            events.push_str(&format!("
{on} => {branch},
            "))
        }
    }

    let k =format!("
impl <{lifetimes} C: 'static + {traits}> WindowBuilder <C> {{
    pub fn create(self) -> Result <(), winit::error::OsError> {{
        let Self(mut data) = self;

        let mut builder = winit::window::WindowBuilder::new();

        {data}

        {requirements}

        let event_loop = EventLoop::with_user_event();

        let winit_window = builder.build(&event_loop)?;

        let mut window_data = WindowData {{
            proxy: event_loop.create_proxy(),
            winit: WinitRef::new(&winit_window)
        }};

        let window = Window::from(&mut window_data);

        {unique_init}

        event_loop.run(move |event, _, cf| {{
            if *cf == ControlFlow::Exit {{
                return
            }}
            *cf = ControlFlow::Wait;

            match event {{
                {events}
                _ => ()
            }}
        }})
    }}
}}
    ");println!("{k}");
    k.parse().unwrap()
}
