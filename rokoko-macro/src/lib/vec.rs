//
// This module provides macros for `rokoko::math::vec`
//

/*
///
/// This macro implements different aliases for `vec`.
///
/// # Usage
///     impl_aliases_for_vec!(<idx_roll_to>; <aliases>)
///
/// # Examples
///
/// ```rust,norun
/// rokoko_macro::impl_aliases_for_vec!(4;
///     b = bool,
///     i = i32,
///     u = u32,
///     f = f32,
///     d = f64
/// );
/// ```
///
#[proc_macro]
#[doc(hidden)]
pub fn impl_aliases_for_vec(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    #[derive(Copy, Clone)]
    struct Alias <'a> {
        alias: &'a str,
        real: &'a str
    }

    struct Iter <'a> {
        to: usize,
        current_to: usize,
        aliases: Vec <Alias <'a>>,
        current_alias: usize
    }

    impl <'a> Iterator for Iter <'a> {
        type Item = String;

        fn next(&mut self) -> Option <Self::Item> {
            if self.current_alias >= self.aliases.len() {
                return None
            }

            let Alias { alias, real } = self.aliases[self.current_alias];
            let current = self.current_to;

            if current == 0 {
                self.current_to = self.to;
                self.current_alias += 1;
                return Some(format!("pub type {alias}vec <const N: usize> = vec <{real}, N>;"))
            }

            self.current_to -= 1;

            Some(format!("pub type {alias}vec{current} = {alias}vec <{current}>;"))
        }
    }

    let iter = {
        if let [to, aliases] = input.split(';').collect::<Vec <_>>()[..] {
            Iter {
                to: to.trim().parse().expect("<integer>"),
                current_to: 0,
                aliases: aliases.split(',').map(|alias| {
                    if let [alias, real] =  alias.split('=').map(str::trim).collect::<Vec <_>>()[..] {
                        Alias { alias, real }
                    } else {
                        panic!("Wrong number of `=`s -> expected <alias> = <real>")
                    }
                }).collect::<Vec <_>>(),
                current_alias: 0
            }
        } else {
            panic!("Expected 2 args: <to>; <aliases>")
        }
    };

    let k = iter.collect::<Vec <_>>().join("\n");
    println!("{k}");
    k.parse().unwrap()
}
 */

///
/// This macro implements different operators on `vec`.
///
/// # Usage
///     impl_bin_ops_for_vec!(<op1> <op2> ...);
///
/// # Examples
/// ```rust,norun
/// //
/// // That one impls:
/// //  1. vec <T, N> + vec <T, N>
/// //  2. vec <T, N> + T
/// //  3. vec <T, N> += vec <T, N>
/// //  4. vec <T, N> += T
/// //
/// // In real use there's probably not only `add`, but also
/// // `sub`, `mul`, `div`, etc.
/// //
/// rokoko_macro::impl_bin_ops_for_vec! {
///     add
/// }
/// ```
///
#[proc_macro]
#[doc(hidden)]
pub fn impl_bin_ops_for_vec(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let ops = input.split_whitespace();
    let mut result = String::new();

    struct EqVariant {
        produce: String,
        assign: String
    }

    impl EqVariant {
        pub fn from_small(small: String) -> Self {
            Self {
                produce: small.clone(),
                assign: small + "_assign"
            }
        }

        pub fn from_big(big: String) -> Self {
            Self {
                produce: big.clone(),
                assign: big + "Assign"
            }
        }
    }

    struct CaseVariant {
        small: EqVariant,
        big: EqVariant
    }

    impl CaseVariant {
        pub fn from_small(small: String) -> Self {
            let mut big = tools::capitalize(&small);

            // I know it is veeeery bad to do such a thing,
            // but it works *here* and `impl_bin_ops_for_vec`
            // won't probably be used elsewhere, so
            // *don't touch it while it works* :)
            if big.starts_with("Bit") {
                let (first, rest) = big.split_at(3);
                big = first.to_string() + &tools::capitalize(rest)
            }

            Self {
                small: EqVariant::from_small(small),
                big: EqVariant::from_big(big)
            }
        }
    }

    for op in ops {
        let CaseVariant {
            small: EqVariant {
                produce: small,
                assign: small_assign
            },
            big: EqVariant {
                produce: big,
                assign: big_assign
            },
        } = CaseVariant::from_small(op.to_string());

        result.push_str(&format!("
///
/// Strange workaround compiler's inability to understand in-place `T::{small}`,
/// but with separate function(which does absolutely the same thing) it *magically* works.
/// Weird.
///
#[nightly(const(T: {big}))]
#[inline(always)]
fn {small} <T: {big}> (a: T, b: T) -> T::Output {{
    T::{small}(a, b)
}}

#[nightly(const(T: {big}))]
impl <T: {big} + Copy, const N: usize> {big} for vec <T, N> {{
    type Output = vec <T::Output, N>;

    #[inline]
    fn {small}(self, rhs: Self) -> Self::Output {{
        self.apply_binary(rhs, {small})
    }}
}}

#[nightly(const(T: {big}))]
impl <T: {big} + Copy, const N: usize> {big} <T> for vec <T, N> {{
    type Output = vec <T::Output, N>;

    #[inline]
    fn {small}(self, rhs: T) -> Self::Output {{
        self.apply_binary_single(rhs, {small})
    }}
}}

#[nightly(const(T: {big} + From <T::Output>))]
impl <T: Copy + {big} + From <T::Output>, const N: usize> {big_assign} for vec <T, N> {{
    fn {small_assign}(&mut self, rhs: Self) {{
        self.modify_binary(rhs, {small})
    }}
}}

#[nightly(const(T: {big} + From <T::Output>))]
impl <T: Copy + {big} + From <T::Output>, const N: usize> {big_assign} <T> for vec <T, N> {{
    fn {small_assign}(&mut self, rhs: T) {{
        self.modify_binary_single(rhs, {small})
    }}
}}
        "))
    }

    result.parse().unwrap()
}

///
/// This macro implements tons of things:
///     - !`NotTuple` auto trait for tuple (if `nightly`)
///     - `Piece` trait for tuple(if `nightly`)
///     - From <vec> for tuple
///     - From <tuple> for vec
///
/// This is one macro and not a separate ones because it would be inconvenient
/// to split.
///
/// # Usage
///     impl_piece_and_not_tuple_and_conversions_to_and_from_vec_for_tuples!(<upper_bound>);
///
#[proc_macro]
#[doc(hidden)]
pub fn impl_not_tuple_and_piece_and_conversions_to_and_from_vec_for_tuples(input: TokenStream) -> TokenStream {
    let to: usize = input.to_string().parse().expect("upper bound");

    // The simplest tuple: `T0, T1, T2, ...`
    let mut tuple = String::new();

    // Piece bounds for tuple: `T0: ~const Piece <T>, T1: ~const Piece <T>, ...`
    let mut piece_bounds = String::new();

    // `N` for `Piece`: `0 + T0::N + T1::N + T2::N + ...`
    let mut piece_n_size = String::from("0");

    // Code to `embed` tuple member
    let mut piece_embed_expr = String::new();

    // Bounds for conversion from `T`: `T0: From <T>, T1: From <T>, ...`
    let mut from_t = String::new();

    // Code to convert vec to tuple
    let mut from_conversions = String::new();

    // Bounds on `T` in conversion from tuple: `T: From <T0> + From <T1> + ...`
    let mut from_tuple_bounds = String::new();

    // Tuple with `Copy` on elements: `T0: Copy, T1: Copy, ...`
    let mut tuple_copy = String::new();

    // Code to convert tuple to vec
    let mut from_tuple_expr = String::new();

    let mut result = String::new();
    for i in 0..=to {
        // Not to convert `i` every time it is used
        let i = i.to_string();

        // `nightly` => `NotTuple` + `Piece`
        if cfg!(feature = "nightly") {
            result.push_str(&format!("
impl <{tuple}> !NotTuple for ({tuple}) {{}}

impl <T: Copy, {piece_bounds}> const Piece <T> for ({tuple}) {{
    const N: usize = {piece_n_size};

    unsafe fn embed(self, mut place: *mut T) {{
        {piece_embed_expr}
    }}
}}
            "))
        }

        result.push_str(&format!("
#[nightly(const({from_t}))]
impl <T: Copy, {from_t}> From <vec <T, {i}>> for ({tuple}) {{
    fn from(x: vec <T, {i}>) -> Self {{
        unsafe {{ ({from_conversions}) }}
    }}
}}

#[nightly(const(T: {from_tuple_bounds}))]
impl <T: {from_tuple_bounds}, {tuple_copy}> From <({tuple})> for vec <T, {i}> {{
    fn from(x: ({tuple})) -> Self {{
        Self([{from_tuple_expr}])
    }}
}}
        "));

        tuple.push_str(&format!("T{i},"));
        piece_bounds.push_str(&format!("T{i}: ~const Piece <T> + Copy,"));
        piece_embed_expr.push_str(&format!("self.{i}.embed(offset(place, {piece_n_size}));"));
        piece_n_size.push_str(&format!("+ T{i}::N"));
        from_t.push_str(&format!("T{i}: From <T>,"));
        from_conversions.push_str(&format!("T{i}::from(*x.get_unchecked({i})),"));
        from_tuple_bounds.push_str(&format!("From <T{i}> +"));
        tuple_copy.push_str(&format!("T{i}: Copy,"));
        from_tuple_expr.push_str(&format!("T::from(x.{i}),"))
    }

    result.parse().unwrap()
}
