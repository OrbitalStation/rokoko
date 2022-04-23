/// Used to add new configuration options to `WindowBuilder`
#[doc(hidden)]
#[macro_export]
macro_rules! config {
    (
        'data: $($( #[$data_meta:meta] )* $data_big:ident $( <( $( $data_generics:tt )* )> )? ($data_low:ident $(: $inner:ty )?), $data_trait:ident ),*
        'events: $($( #[$events_meta:meta] )* $low:ident($( $ty:ty ),*) $( -> $out:ty )?, $big:ident, $event_trait:ident ),*
        'impl: impl <C: Config> $( $tt:tt )*
    ) => {
        $(
            $( #[$data_meta] )*
            pub struct $data_big $( < $( $data_generics )* > )? $( ($inner) )?;

            pub trait $data_trait $( < $( $data_generics )* > )? {
                fn $data_low (&self) -> Option <&$data_big $( < $( $data_generics )* > )? >;
            }

            impl <$( $( $data_generics )*, )? C: ~const GetData <$data_big $( < $( $data_generics )* > )?>> const $data_trait $( < $( $data_generics )* > )? for C {
                #[inline(always)]
                fn $data_low (&self) -> Option <&$data_big $( < $( $data_generics )* > )? > {
                    self.get()
                }
            }

            impl <C> WindowBuilder <C> {
                config!(@data-low ($( #[$data_meta] )*) $data_low, $data_big, ($( $( $data_generics )* )?), $( $inner )?);
            }
        )*

        $(
            $( #[$events_meta] )*
            pub struct $big;

            pub trait $event_trait : GetFn <$big> {
                fn $low (&mut self) -> Option <&mut Self::Type>;
            }

            impl <C: ~const GetFn <$big>> const $event_trait for C {
                #[inline(always)]
                fn $low (&mut self) -> Option <&mut Self::Type> {
                    self.get()
                }
            }

            impl Callback for $big {
                type Output = config!(@ty $( $out )?);

                type Args = ($( $ty ),* ,);
            }

            impl <C> WindowBuilder <C> {
                $( #[$events_meta] )*
                pub const fn $low <F: FnMut <<$big as Callback>::Args, Output = <$big as Callback>::Output>> (self, cb: F)
                    -> WindowBuilder <With <OnEventFnContainer <$big, F>, C>> {
                    self.on_event::<$big, F>(cb)
                }
            }
        )*

        config!(@create ('impl $( $tt )* ), ('generics $( $( $( $data_generics )*, )? )*), ('traits), ('cb $( $big )* ), ('data $( ($data_big $( < $( $data_generics )* > )?) )*));
    };

    (@data-low ($( $meta:tt )*) $low:ident, $big:ident, ($( $generics:tt )*), $x:ty) => {
        $( $meta )*
        pub const fn $low < $( $generics )* > (self, x: $x) -> WindowBuilder <With <$big, C>> {
            WindowBuilder(With {
                data: $big(x),
                next: self.to_inner()
            })
        }
    };

    (@data-low ($( $meta:tt )*) $low:ident, $big:ident, ($( $generics:tt )*),) => {
        $( $meta )*
        pub const fn $low < $( $generics )* > (self) -> WindowBuilder <With <$big, C>> {
            WindowBuilder(With {
                data: $big,
                next: self.to_inner()
            })
        }
    };

    (@create ('impl $( $impl:tt )*), ('generics $( $generics:tt )*), ('traits $( $tt:tt )*), ('cb $trait:ident $( $traits:ident )* ), ('data $( $data:tt )*)) => {
        config!(@create ('impl $( $impl )*), ('generics $( $generics )*), ('traits GetFn <$trait> + $( $tt )*), ('cb $( $traits )*), ('data $( $data )*));
    };

    (@create ('impl $( $impl:tt )*), ('generics $( $generics:tt )*), ('traits $( $tt:tt )*), ('cb), ('data ($( $trait:tt )*) $( $traits:tt )*)) => {
        config!(@create ('impl $( $impl )*), ('generics $( $generics )*), ('traits GetData <$( $trait )*> + $( $tt )*), ('cb), ('data $( $traits )* ));
    };

    (@create ('impl $( $impl:tt )*), ('generics $( $generics:tt )*), ('traits $( $tt:tt )*), ('cb), ('data)) => {
        // A small trick here since writing `~const` in labels above would be treated as an error(why?),
        // so using `#[nightly]` to make all traits `const` instead.
        #[nightly(const(C: $( $tt )*))]
        impl <C: 'static + $( $tt )*, $( $generics )*> $( $impl )*
    };

    (@ty) => { () };

    (@ty $ty:ty) => { $ty }
}
