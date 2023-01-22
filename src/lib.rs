/// Helper to `impl From<$src_type> for $dst_type`.
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
/// from!(struct (src: StructA) -> StructB {
///     num,
///     text: format!("num = {}", src.num),
/// });
/// assert_eq!(StructB { num: 123, text: "num = 123".into() }, StructA { num: 123 }.into());
///
/// // convert enum to enum
/// from!(enum (src: EnumA) -> EnumB {
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
        impl From<$src_type> for $dst_type {
            fn from($src: $src_type) -> Self $block
        }
    };

    // utility for struct fields
    (STRUCT_FIELD $src:ident.$field:ident) => { $src.$field.into() };
    (STRUCT_FIELD $src:ident.$field:ident => $value:expr) => { $value };

    // convert struct type
    (struct ($src:ident : $src_type:ty) -> $dst_type:ty { $($field:ident$(: $value:expr)?),*$(,)? }) => {
        from!(($src: $src_type) -> $dst_type {
            Self {
                $($field: from!(STRUCT_FIELD $src.$field $(=> $value)?)),*
            }
        });
    };

    // utility for enum variants
    (ENUM_VARIANT $variant:ident $(($var:ident))?) => { Self::$variant$(($var.into()))?  };
    (ENUM_VARIANT $variant:ident $(($var:ident))? => $value:expr) => { $value };

    // convert enum type
    (enum ($src:ident : $src_type:ty) -> $dst_type:ty { $($variant:ident$(($var:ident))?$(=> $value:expr)?),*$(,)? }) => {
        from!(($src: $src_type) -> $dst_type {
            type Src = $src_type;
            match $src {
                $(Src::$variant$(($var))? => from!(ENUM_VARIANT $variant$(($var))? $(=> $value)?)),*
            }
        });
    };
}

// ------------------------------------------------------------

/// Helper to `impl TryFrom<$src_type> for $dst_type`.
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
/// try_from!(struct (src: StructA) -> <StructB, anyhow::Error> {
///     num,
///     text: format!("num = {}", src.num),
/// });
/// assert_eq!(StructB { num: 123, text: "num = 123".into() }, StructA { num: 123 }.try_into().unwrap());
///
/// // convert enum to enum
/// try_from!(enum (src: EnumA) -> <EnumB, anyhow::Error> {
///     Case1,
///     Case2(n),
///     Case3(x),
///     Case4(s) => anyhow::bail!("error"),
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
        impl TryFrom<$src_type> for $dst_type {
            type Error = $err_type;
            fn try_from($src: $src_type) -> Result<Self, Self::Error> $block
        }
    };

    // utility for struct fields
    (STRUCT_FIELD $src:ident.$field:ident) => { $src.$field.try_into()?  };
    (STRUCT_FIELD $src:ident.$field:ident => $value:expr) => { $value };

    // convert struct type
    (struct ($src:ident : $src_type:ty) -> <$dst_type:ty, $err_type:ty> { $($field:ident$(: $value:expr)?),*$(,)? }) => {
        try_from!(($src: $src_type) -> <$dst_type, $err_type> {
            Ok(Self {
                $($field: try_from!(STRUCT_FIELD $src.$field $(=> $value)?),)*
            })
        });
    };

    // utility for enum variants
    (ENUM_VARIANT $variant:ident $(($var:ident))?) => { Self::$variant$(($var.try_into()?))?  };
    (ENUM_VARIANT $variant:ident $(($var:ident))? => $value:expr) => { $value };

    // convert enum type
    (enum ($src:ident : $src_type:ty) -> <$dst_type:ty, $err_type:ty> { $($variant:ident$(($var:ident))?$(=> $value:expr)?),*$(,)? }) => {
        try_from!(($src: $src_type) -> <$dst_type, $err_type> {
            type Src = $src_type;
            Ok(match $src {
                $(Src::$variant$(($var))? => try_from!(ENUM_VARIANT $variant$(($var))? $(=> $value)?),)*
            })
        });
    };
}

// ----------------------------------------------------------------

#[cfg(feature = "warned")]
pub use warned;

/// Helper to `impl ForceFrom<$src_type> for $dst_type`.
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
/// // convert struct to struct
/// force_from!(struct (src: StructA) -> <StructB, anyhow::Error> {
///     num,
///     text: format!("num = {}", src.num),
/// });
/// assert_eq!(StructB { num: 123, text: "num = 123".into() }, StructA { num: 123 }.force_into().value);
///
/// // convert enum to enum
/// force_from!(enum (src: EnumA) -> <EnumB, anyhow::Error> {
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
/// ```
#[cfg(feature = "warned")]
#[macro_export]
macro_rules! force_from {
    // impl ForceFrom<$src_type> for $dst_type
    (($src:ident : $src_type:ty) -> <$dst_type:ty, $warn_type:ty> $block:block) => {
        impl warned::ForceFrom<$src_type> for $dst_type {
            type Warning = $warn_type;
            fn force_from($src: $src_type) -> warned::Warned<Self, Self::Warning> $block
        }
    };

    // utilities for struct fields
    (STRUCT_FIELD $src:ident.$field:ident, $warnings:ident) => {
        warned::Warned::unwrap($src.$field.force_into(), &mut $warnings)
    };
    (STRUCT_FIELD $src:ident.$field:ident, $warnings:ident => $value:expr) => {
        $value
    };

    // convert struct type
    (struct ($src:ident : $src_type:ty) -> <$dst_type:ty, $warn_type:ty> { $($field:ident$(: $value:expr)?),*$(,)? }) => {
        force_from!(($src: $src_type) -> <$dst_type, $warn_type> {
            let mut warnings: Vec<$warn_type> = vec![];
            let value = Self {
                $($field: force_from!(STRUCT_FIELD $src.$field, warnings $(=> $value)?),)*
            };
            warned::Warned::new(value, warnings)
        });
    };

    // utilities for enum variants
    (ENUM_VARIANT $variant:ident) => { Self::$variant.into() };
    (ENUM_VARIANT $variant:ident($var:ident)) => {
        warned::Warned::map(
            warned::Warned::map_warnings($var.force_into(), Into::into),
            Self::$variant,
        )
    };
    (ENUM_VARIANT $variant:ident $(($var:ident))? => $value:expr) => { $value };

    // convert enum type
    (enum ($src:ident : $src_type:ty) -> <$dst_type:ty, $warn_type:ty> { $($variant:ident$(($var:ident))?$(=> $value:expr)?),*$(,)? }) => {
        force_from!(($src: $src_type) -> <$dst_type, $warn_type> {
            type Src = $src_type;
            match $src {
                $(Src::$variant$(($var))? => force_from!(ENUM_VARIANT $variant$(($var))? $(=> $value)?),)*
            }
        });
    };
}
