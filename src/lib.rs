/// DON'T USE! This can only be used within the [from] macro.
#[doc(hidden)]
#[macro_export]
macro_rules! __from_struct_field {
    ($src:ident.$field:ident) => {
        $src.$field.into()
    };
    ($src:ident.$field:ident => $value:expr) => {
        $value
    };
}

/// DON'T USE! This can only be used within the [from] macro.
#[doc(hidden)]
#[macro_export]
macro_rules! __from_enum_variant {
    ($variant:ident $(($var:ident))?) => { Self::$variant$(($var.into()))? };
    ($variant:ident $(($var:ident))? => $value:expr) => { $value };
}

/// Helper to `impl From<$src_type> for $dst_type`.
///
/// # Example
/// ```
/// use impl_converter_helper::*;
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct StructA { num: i32 }
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct StructB { num: i64, text: String }
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum EnumA { Case1, Case2(i32), Case3(StructA), Case4(String) }
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum EnumB { Case1, Case2(i64), Case3(StructB), CaseX(String) }
///
/// // convert struct to struct
/// from!((src: StructA) -> StructB as struct {
///     num,
///     text: format!("num = {}", src.num),
/// });
/// assert_eq!(StructB { num: 123, text: "num = 123".into() }, StructA { num: 123 }.into());
///
/// // convert enum to enum
/// from!((src: EnumA) -> EnumB as enum {
///     Case1,
///     Case2(n),
///     Case3(x),
///     Case4(s) => Self::CaseX(s),
/// });
/// assert_eq!(EnumB::Case2(321), EnumA::Case2(321).into());
///
/// // convert anyway
/// from!((src: StructA) -> EnumA {
///     Self::Case2(src.num)
/// });
/// assert_eq!(EnumA::Case2(111), StructA { num: 111 }.into());
/// ```
#[macro_export]
macro_rules! from {
    // impl From<$src_type> for $dst_type
    (($src:ident : $src_type:ty) -> $dst_type:ty $block:block) => {
        impl ::std::convert::From<$src_type> for $dst_type {
            fn from($src: $src_type) -> Self $block
        }
    };

    // convert struct type
    (($src:ident : $src_type:ty) -> $dst_type:ty as struct { $($field:ident$(: $value:expr)?),*$(,)? }) => {
        from!(($src: $src_type) -> $dst_type {
            Self {
                $($field: __from_struct_field!($src.$field $(=> $value)?)),*
            }
        });
    };

    // convert enum type
    (($src:ident : $src_type:ty) -> $dst_type:ty as enum { $($variant:ident$(($var:ident))?$(=> $value:expr)?),*$(,)? }) => {
        from!(($src: $src_type) -> $dst_type {
            type Src = $src_type;
            match $src {
                $(Src::$variant$(($var))? => __from_enum_variant!($variant$(($var))? $(=> $value)?)),*
            }
        });
    };
}

// ------------------------------------------------------------

/// DON'T USE! This can only be used within the [try_from] macro.
#[doc(hidden)]
#[macro_export]
macro_rules! __try_from_struct_field {
    ($src:ident.$field:ident) => {
        $src.$field.try_into()?
    };
    ($src:ident.$field:ident => $value:expr) => {
        $value
    };
}

/// DON'T USE! This can only be used within the [try_from] macro.
#[doc(hidden)]
#[macro_export]
macro_rules! __try_from_enum_variant {
    ($variant:ident $(($var:ident))?) => { Ok(Self::$variant$(($var.try_into()?))?)  };
    ($variant:ident $(($var:ident))? => $value:expr) => { $value };
}

/// Helper to `impl TryFrom<$src_type> for $dst_type`.
///
///
/// # Example
/// ```
/// use impl_converter_helper::*;
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct StructA { num: i32 }
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct StructB { num: i64, text: String }
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum EnumA { Case1, Case2(i32), Case3(StructA), Case4(String) }
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum EnumB { Case1, Case2(i64), Case3(StructB) }
///
/// // convert struct to struct
/// try_from!((src: StructA) -> <StructB, anyhow::Error> as struct {
///     num,
///     text: format!("num = {}", src.num),
/// });
/// assert_eq!(StructB { num: 123, text: "num = 123".into() }, StructA { num: 123 }.try_into().unwrap());
///
/// // convert enum to enum
/// try_from!((src: EnumA) -> <EnumB, anyhow::Error> as enum {
///     Case1,
///     Case2(n),
///     Case3(x),
///     Case4(s) => Err(anyhow::anyhow!("error")),
/// });
/// assert_eq!(EnumB::Case2(321), EnumA::Case2(321).try_into().unwrap());
///
/// // convert anyway
/// try_from!((src: StructA) -> <EnumA, anyhow::Error> {
///     Ok(Self::Case2(src.num))
/// });
/// assert_eq!(EnumA::Case2(111), StructA { num: 111 }.try_into().unwrap());
/// ```
#[macro_export]
macro_rules! try_from {
    // impl From<$src_type> for $dst_type
    (($src:ident : $src_type:ty) -> <$dst_type:ty, $err_type:ty> $block:block) => {
        impl ::std::convert::TryFrom<$src_type> for $dst_type {
            type Error = $err_type;
            fn try_from($src: $src_type) -> ::std::result::Result<Self, Self::Error> $block
        }
    };

    // convert struct type
    (($src:ident : $src_type:ty) -> <$dst_type:ty, $err_type:ty> as struct { $($field:ident$(: $value:expr)?),*$(,)? }) => {
        try_from!(($src: $src_type) -> <$dst_type, $err_type> {
            Ok(Self {
                $($field: __try_from_struct_field!($src.$field $(=> $value)?),)*
            })
        });
    };

    // convert enum type
    (($src:ident : $src_type:ty) -> <$dst_type:ty, $err_type:ty> as enum { $($variant:ident$(($var:ident))?$(=> $value:expr)?),*$(,)? }) => {
        try_from!(($src: $src_type) -> <$dst_type, $err_type> {
            type Src = $src_type;
            match $src {
                $(Src::$variant$(($var))? => __try_from_enum_variant!($variant$(($var))? $(=> $value)?),)*
            }
        });
    };
}

// ----------------------------------------------------------------

#[cfg(feature = "warned")]
pub use warned;

/// DON'T USE! This can only be used within the [force_from] macro.
#[cfg(feature = "warned")]
#[doc(hidden)]
#[macro_export]
macro_rules! __force_from_struct_field {
    ($src:ident.$field:ident, $warnings:ident) => {
        $crate::warned::Warned::unwrap($src.$field.force_into(), &mut $warnings)
    };
    ($src:ident.$field:ident, $warnings:ident => @warn $value:expr) => {
        $crate::warned::Warned::unwrap($value, &mut $warnings)
    };
    ($src:ident.$field:ident, $warnings:ident => $value:expr) => {
        $value
    };
}

/// DON'T USE! This can only be used within the [force_from] macro.
#[cfg(feature = "warned")]
#[doc(hidden)]
#[macro_export]
macro_rules! __force_from_enum_variant {
    // utilities for enum variants
    ($variant:ident) => {
        Self::$variant.into()
    };
    ($variant:ident($var:ident)) => {
        $crate::warned::Warned::map(
            $crate::warned::Warned::map_warnings($var.force_into(), Into::into),
            Self::$variant,
        )
    };
    ($variant:ident $(($var:ident))? => $value:expr) => {
        $crate::warned::Warned::map_warnings($value, Into::into)
    };
}

/// Helper to `impl ForceFrom<$src_type> for $dst_type`.
///
/// # Example
/// ```
/// use impl_converter_helper::*;
/// use impl_converter_helper::warned::ForceInto;
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct StructA { num: i32 }
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct StructB { num: i64, text: String }
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum EnumA { Case1, Case2(i32), Case3(StructA), Case4(String) }
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum EnumB { Case1, Case2(i64), Case3(StructB) }
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct CollectionA { items: Vec<EnumA> };
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct CollectionB { items: Vec<EnumB> };
///
/// // convert struct to struct
/// force_from!((src: StructA) -> <StructB, anyhow::Error> as struct {
///     num,
///     text: format!("num = {}", src.num),
/// });
/// assert_eq!(StructB { num: 123, text: "num = 123".into() }, StructA { num: 123 }.force_into().value);
///
/// // convert enum to enum
/// force_from!((src: EnumA) -> <EnumB, anyhow::Error> as enum {
///     Case1,
///     Case2(n),
///     Case3(x),
///     Case4(s) => warned::Warned::new(Self::Case1, vec![anyhow::anyhow!("fallback to Case1")])
/// });
/// assert_eq!(EnumB::Case2(321), EnumA::Case2(321).force_into().value);
///
/// // convert anyway
/// force_from!((src: StructA) -> <EnumA, anyhow::Error> {
///     warned::Warned::new(Self::Case2(src.num), vec![])
/// });
/// assert_eq!(EnumA::Case2(111), StructA { num: 111 }.force_into().value);
///
/// force_from!((src: CollectionA) -> <CollectionB, anyhow::Error> as struct {
///     items: @warn src.items.into_iter().map(ForceInto::force_into).collect()
/// });
/// ```
#[cfg(feature = "warned")]
#[macro_export]
macro_rules! force_from {
    // impl ForceFrom<$src_type> for $dst_type
    (($src:ident : $src_type:ty) -> <$dst_type:ty, $warn_type:ty> $block:block) => {
        impl $crate::warned::ForceFrom<$src_type> for $dst_type {
            type Warning = $warn_type;
            fn force_from($src: $src_type) -> $crate::warned::Warned<Self, Self::Warning> $block
        }
    };

    // convert struct type
    (($src:ident : $src_type:ty) -> <$dst_type:ty, $warn_type:ty> as struct { $($field:ident$(: $(@$warn:ident)? $value:expr)?),*$(,)? }) => {
        force_from!(($src: $src_type) -> <$dst_type, $warn_type> {
            let mut warnings: Vec<$warn_type> = vec![];
            let value = Self {
                $($field: __force_from_struct_field!($src.$field, warnings $(=> $(@$warn)? $value)?),)*
            };
            $crate::warned::Warned::new(value, warnings)
        });
    };

    // convert enum type
    (($src:ident : $src_type:ty) -> <$dst_type:ty, $warn_type:ty> as enum { $($variant:ident$(($var:ident))?$(=> $value:expr)?),*$(,)? }) => {
        force_from!(($src: $src_type) -> <$dst_type, $warn_type> {
            type Src = $src_type;
            match $src {
                $(Src::$variant$(($var))? => __force_from_enum_variant!($variant$(($var))? $(=> $value)?),)*
            }
        });
    };
}
