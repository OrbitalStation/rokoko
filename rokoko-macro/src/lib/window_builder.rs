//
// This module provides macros for the `WindowBuilder`
//

///
/// Describes data to be added to `WindowBuilder`.
///
/// # Side effects
///
/// Adds data to `TRAITS` & `LIFETIMES` in `wb_statics`
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
            attrs,
            ident,
            ty
        } = field;

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
///
/// Adds data to `TRAITS` in `wb_statics`
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

    /// A type that have a name during parsing but loses it later
    struct TypeWithName(Box <Type>);

    impl Parse for TypeWithName {
        fn parse(input: ParseStream) -> syn::Result <Self> {
            if input.peek2(Token![:]) {
                let _: Ident = input.parse()?;
                let _: Token![:] = input.parse()?;
            }
            Ok(Self(Box::new(input.parse()?)))
        }
    }

    /// A callback to be added to `WindowBuilder`
    struct Callback {
        attrs: Vec <Attribute>,
        ident: String,
        args: Punctuated <TypeWithName, Token![,]>,
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
            attrs,
            ident,
            args,
            ret
        } = cb;

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
            .map(|TypeWithName(ty)| ty.to_token_stream().to_string())
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
/// Transforms `C: Config` on impl to set of all traits created by
/// `data` and `events` sections
///
/// # Side effects
///
/// Flushes both `TRAITS` & `LIFETIMES` in `wb_statics`
///
#[proc_macro_attribute]
#[doc(hidden)]
pub fn window_builder_create(_: TokenStream, input: TokenStream) -> TokenStream {
    const CONFIG: &'static str = "Config";
    const CFG_LEN: usize = CONFIG.len();

    let mut input = input.to_string();

    let generics = input.find('<').unwrap() + 1;
    let c = input.find(CONFIG).unwrap();
    input.drain(c..c + CFG_LEN);

    input.insert_str(c, &wb_statics::traits());
    input.insert_str(c, "'static +");
    input.insert_str(generics, &wb_statics::lifetimes());

    input.parse().unwrap()
}
