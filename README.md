# Helper Macro Library to Implement Converters

This crate is a declarative macro library to help you implement the `From` or `TryFrom` trait for your type.

## Implementing `From` trait
The `from` macro helps you to implement `From` trait.

```rust
from!((src: SourceType) -> TargetType {
    ...
});
```
The above code results in the following code.
```rust
impl From<SourceType> for TargetType {
    fn from(src: SourceType) -> Self {
        ...
    }
}
```

You can also use the `as struct` or `as enum` keywords to convert between `struct` types or `enum` types.  Some examples are shown below.

```rust
use impl_converter_helper::*;

#[derive(Debug, PartialEq, Eq)]
struct StructA { num: i32 }

#[derive(Debug, PartialEq, Eq)]
struct StructB { num: i64, text: String }

#[derive(Debug, PartialEq, Eq)]
enum EnumA { Case1, Case2(i32), Case3(StructA), Case4(String) }

#[derive(Debug, PartialEq, Eq)]
enum EnumB { Case1, Case2(i64), Case3(StructB), CaseX(String) }

// convert struct to struct
from!((src: StructA) -> StructB as struct {
    num,
    text: format!("num = {}", src.num),
});

assert_eq!(StructB { num: 123, text: "num = 123".into() }, StructA { num: 123 }.into());

// convert enum to enum
from!((src: EnumA) -> EnumB as enum {
    Case1,
    Case2(n),
    Case3(x),
    Case4(s) => Self::CaseX(s),
});

assert_eq!(EnumB::Case2(321), EnumA::Case2(321).into());

// convert anyway
from!((src: StructA) -> EnumA {
    Self::Case2(src.num)
});

assert_eq!(EnumA::Case2(111), StructA { num: 111 }.into());
```

## Implementing `TryFrom` trait
The `try_from` macro helps you to implement `TryFrom` trait.

```rust
try_from!((src: SourceType) -> <TargetType, ErrorType> {
    ...
});
```
The above code results in the following code.
```rust
impl TryFrom<SourceType> for TargetType {
    type Error = ErrorType;
    fn try_from(src: SourceType) -> Result<Self, Self::Error> {
        ...
    }
}
```

You can also use the `as struct` or `as enum` keywords to convert between `struct` types or `enum` types.  Some examples are shown below.

```rust
use impl_converter_helper::*;

#[derive(Debug, PartialEq, Eq)]
struct StructA { num: i32 }

#[derive(Debug, PartialEq, Eq)]
struct StructB { num: i64, text: String }

#[derive(Debug, PartialEq, Eq)]
enum EnumA { Case1, Case2(i32), Case3(StructA), Case4(String) }

#[derive(Debug, PartialEq, Eq)]
enum EnumB { Case1, Case2(i64), Case3(StructB) }

// convert struct to struct
try_from!((src: StructA) -> <StructB, anyhow::Error> as struct {
    num,
    text: format!("num = {}", src.num),
});

assert_eq!(StructB { num: 123, text: "num = 123".into() }, StructA { num: 123 }.try_into().unwrap());

// convert enum to enum
try_from!((src: EnumA) -> <EnumB, anyhow::Error> as enum {
    Case1,
    Case2(n),
    Case3(x),
    Case4(s) => Err(anyhow::anyhow!("error")),
});

assert_eq!(EnumB::Case2(321), EnumA::Case2(321).try_into().unwrap());

// convert anyway
try_from!((src: StructA) -> <EnumA, anyhow::Error> {
    Ok(Self::Case2(src.num))
});

assert_eq!(EnumA::Case2(111), StructA { num: 111 }.try_into().unwrap());
```
