///
/// Provides different commands to do if `nightly` feature is used/not used.
///
/// # Examples
/// ```rust,norun
/// /// This function will be available only if `nightly` feature is enabled.
/// #[nightly]
/// fn some_func() {}
///
/// /// This function will be `const` if `nightly` feature is
/// /// enabled and not `const` otherwise
/// #[nightly(const)]
/// fn some_func2() {}
///
/// /// If `nightly` feature is enabled, then this
/// /// function becomes `const` and `T` now is
/// /// `T: ~const Default`.
/// /// Nothing happens otherwise.
/// #[nightly(const(T: Default))]
/// fn some_func3 <T: Default> () -> T {
///     T::default()
/// }
/// ```
///
#[proc_macro_attribute]
pub fn nightly(args: TokenStream, input: TokenStream) -> TokenStream {
    ///
    /// Represents command of `nightly`.
    ///
    struct Cmd {
        name: &'static str,
        func: fn(args: &str, input: TokenStream) -> TokenStream
    }

    ///
    /// Convenient set of commands
    ///
    macro_rules! cmds {
        ($( $name:literal : $func:expr ),*) => {
            const CMDS: [Cmd; cmds!(@len $( $name )*)] = [$( Cmd {
                name: $name,
                func: $func
            } ),*];
        };

        (@len) => {
            0
        };

        (@len $i:literal $( $ii:literal )*) => {
            1 + cmds!(@len $( $ii )*)
        };
    }

    ///
    /// If `nightly` feature is used, then
    /// it makes inner function/impl `const`.
    /// Does nothing otherwise.
    ///
    /// Note: `args` can specify const requirements on specific generics.
    /// WARNING! Ignores `where` clause.
    ///
    /// # Examples
    /// Here if `nightly`
    /// feature is used then item becomes const and bound `Default` on generic `T` now
    /// needs to be const as well
    ///  ```norun
    ///     #[nightly(const(T: Default))]
    ///  ```
    ///
    ///  More examples:
    ///  ```norun
    ///     #[nightly(const(T: Copy + Clone))]
    ///     #[nightly(const(T: ATrait, U: SomeOtherTrait + OneAnother))]
    ///  ```
    ///
    #[cfg(feature = "nightly")]
    fn r#const(args: &str, input: TokenStream) -> TokenStream {
        use syn::__private::ToTokens;

        #[derive(Default)]
        struct Generics {
            open: usize,
            close: usize
        }

        impl Generics {
            ///
            /// Finds generics in code
            ///
            pub fn find(input: &str) -> Option <Self> {
                let mut counter = 1u8;
                // We need this to filter `->`, where `>` is not a paired with `<`
                let mut previous_was_minus = false;
                let opening_bracket = input.find('<')? + 1;
                let closing_bracket = input[opening_bracket..].find(move |c: char| -> bool {
                    if c == '<' {
                        counter += 1
                    } else if c == '>' && !previous_was_minus {
                        counter -= 1
                    } else {
                        previous_was_minus = c == '-'
                    }

                    counter == 0
                })? + opening_bracket;

                Some(Generics {
                    open: opening_bracket,
                    close: closing_bracket
                })
            }

            ///
            /// Returns true if generics exist, false otherwise
            ///
            pub fn exist(&self) -> bool {
                self.open != self.close
            }
        }

        //
        // Takes string `input`, finds generics and adds `const` requirements.
        //
        let parse_and_add_requirements = move |input: String, precalc_generics: Generics| -> TokenStream {
            if !precalc_generics.exist() || args.is_empty() {
                return input.parse().unwrap()
            }

            #[derive(Debug)]
            struct Requirement {
                generic: String,
                traits: Vec <String>
            }

            ///
            /// Parse generics to vector of requirements
            ///
            fn get_trait_reqs(generics: &str, sfinae: bool) -> Vec <Requirement> {
                if generics.is_empty() {
                    return Vec::new()
                }

                let mut generics = generics.trim();

                if generics.ends_with(',') {
                    generics = &generics[..generics.len() - 1]
                }

                // This is needed because when `TokenStream` is converted into `String`,
                // newlines are automatically placed after some column,
                // and that ruins everything, because if generic in `args` is
                // `T: Default` and in item `T:\nDefault`, they will not match.
                let generics = generics.replace(char::is_whitespace, " ");

                ///
                /// Just like `input.split_terminator(separator).filter_map(filter_map).collect()`,
                /// but also counts separators in different brackets
                ///
                fn split_with_brackets <'a, T> (
                    input: &'a str,
                    separator: char,
                    brackets: &[core::ops::Range <char>],
                    filter_map: impl Fn(&'a str) -> Option <T>
                ) -> Vec <T> {
                    let mut parts = Vec::new();
                    let mut current = 0..0usize;

                    let brackets =
                        brackets.iter().map(|b| b.start).collect::<Vec <_>>()
                            ..
                        brackets.iter().map(|b| b.end).collect::<Vec <_>>();

                    let mut insideness_level = 0;

                    for (idx, char) in input.chars().enumerate() {
                        if brackets.start.contains(&char) {
                            insideness_level += 1
                        } else if brackets.end.contains(&char) && insideness_level != 0 {
                            insideness_level -= 1
                        }
                        if char == separator && insideness_level == 0 {
                            if let Some(x) = filter_map(&input[current]) {
                                parts.push(x)
                            }
                            current = idx + char.len_utf8()..idx + char.len_utf8()
                        } else {
                            current.end += char.len_utf8()
                        }
                    }
                    if let Some(x) = filter_map(&input[current]) {
                        parts.push(x)
                    }

                    parts
                }

                split_with_brackets(
                    &generics,
                    ',',
                    &[
                        '('..')',
                        '['..']',
                        '{'..'}',
                        '<'..'>'
                    ],
                    |req| {
                        if req.is_empty() {
                            return None
                        }

                        let (generic, traits) = match req.split_once(':') {
                            Some(ok) => ok,
                            None if sfinae => return Some(Requirement {
                                generic: req.to_string(),
                                traits: Vec::new()
                            }),
                            None => panic!("Expected requirement of form <generic>: <traits>")
                        };
                        Some(Requirement {
                            generic: generic.trim().to_string(),
                            traits: traits
                                .split_terminator('+')
                                .map(str::trim)
                                .map(str::to_string)
                                .collect()
                        })
                    }
                )
            }

            let const_reqs = get_trait_reqs(args, false);

            let item_reqs = get_trait_reqs(&input[precalc_generics.open..precalc_generics.close], true);

            let mut generics = String::new();
            for Requirement { generic, mut traits } in item_reqs {
                for t in &mut traits {
                    if const_reqs
                        .iter()
                        .find(|req| req.generic == generic)
                        .map(|req| req.traits.contains(t))
                        .unwrap_or_default() {
                        *t = format!("~const {t}")
                    }
                }
                generics.push_str(&format!("{generic}: {},", traits.join("+")))
            }
            (input[..precalc_generics.open].to_string() + &generics + &input[precalc_generics.close..]).parse().unwrap()
        };

        match syn::parse_macro_input::parse::<syn::ItemFn>(input.clone()) {
            // `input` = function
            Ok(mut f) => {
                f.sig.constness = Some(Default::default());
                let f = f.to_token_stream().to_string();
                let generics = Generics::find(&f).unwrap_or_default();
                parse_and_add_requirements(f, generics)
            },
            Err(_) => match syn::parse_macro_input::parse::<syn::ItemImpl>(input) {
                // `input` = impl block
                Ok(i) => {
                    let mut code = i.to_token_stream().to_string();
                    let generics = if i.trait_.is_some() {
                        // Trait impl
                        let generics = Generics::find(&code[..code.find("for").unwrap()]).unwrap_or_default();
                        let t = if generics.exist() {
                            generics.close + 1
                        } else {
                            code.find("impl").unwrap() + 4
                        };
                        code = code[..t].to_string() + " const" + &code[t..];
                        generics
                    } else {
                        // No trait
                        Generics::find(code[code.find("impl").unwrap() + 4..].trim()).unwrap_or_default()
                    };
                    parse_and_add_requirements(code, generics)
                },
                // Neither `fn` nor `impl`
                Err(e) => TokenStream::from(e.to_compile_error())
            }
        }
    }

    ///
    /// If `nightly` feature is used, then
    /// it makes inner function/impl `const`.
    /// Does nothing otherwise.
    ///
    /// Note: `args` can specify const requirements on specific generics.
    /// WARNING! Ignores `where` clause.
    ///
    /// # Examples
    /// Here if `nightly`
    /// feature is used then item becomes const and bound `Default` on generic `T` now
    /// needs to be const as well
    ///  ```norun
    ///     #[nightly(const(T: Default))]
    ///  ```
    ///
    ///  More examples:
    ///  ```norun
    ///     #[nightly(const(T: Copy + Clone))]
    ///     #[nightly(const(T: ATrait, U: SomeOtherTrait + OneAnother))]
    ///  ```
    ///
    #[cfg(not(feature = "nightly"))]
    fn r#const(_: &str, input: TokenStream) -> TokenStream {
        input
    }

    cmds! {
        // The difference between `const` and `const_force` is that
        // if `nightly` is not enabled, `const` will leave item as it is,
        // and `const_force` will remove it.
        "const_force": |args, input| {
            if cfg!(feature = "nightly") {
                r#const(args, input)
            } else {
                TokenStream::new()
            }
        },
        "const": r#const
    }

    ///
    /// Called if `#[nightly]` is used.
    ///
    /// Removes `item` if `nightly` feature is not used.
    ///
    #[cfg(feature = "nightly")]
    fn enable_if(item: TokenStream) -> TokenStream {
        item
    }

    ///
    /// Called if `#[nightly]` is used.
    ///
    /// Removes `item` if `nightly` feature is not used.
    ///
    #[cfg(not(feature = "nightly"))]
    fn enable_if(_: TokenStream) -> TokenStream {
        TokenStream::new()
    }

    if args.is_empty() {
        enable_if(input)
    } else {
        let args = args.to_string();
        let args = args.trim();

        if let Some(cmd) = CMDS.into_iter().find(|cmd| args.starts_with(cmd.name)) {
            let mut args = args[cmd.name.len()..].trim();

            if !args.is_empty() {
                assert_eq!(args.chars().next().expect("arguments"), '(', "subcommand's args should be enclosed in parentheses");
                assert_eq!(args.chars().next_back().expect("arguments"), ')', "subcommand's args should be enclosed in parentheses");
                args = &args[1..args.len() - 1];
            }

            (cmd.func)(args, input)
        } else {
            panic!("no such command: `{}`", match args.find('(') {
                Some(x) => &args[..x],
                None => &args
            })
        }
    }
}
