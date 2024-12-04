#[cfg(feature = "serde")]
use ::tracing::Level;

/// Create an enum and assign variant strings.
///
/// Implements [`std::str::FromStr`] and optionally [`core::fmt::Display`] for the enum.
///
/// Note: This does not declare the enum. For use in serde, you will have to put something like
/// `#[serde(rename_all = "snake_case")]` on the enum, or `#[serde(rename = "my-variant")]` on a specific variant.
/// ```
/// use vlk_tracing_subscriber::stringable_enum;
///
/// pub enum MyEnum {
///     Foo,
///     Bar,
/// }
///
/// stringable_enum! {
///     MyEnum {
///         Foo = "foo",
///         Bar = "bar",
///     }
/// }
///
/// pub enum NoDisplayEnum {
///     Baz,
///     Quux,
///     OtherGibberish,
/// }
///
/// stringable_enum! {
///     @no-display
///     NoDisplayEnum {
///         Baz = "baz",
///         Quux = "quux",
///         OtherGibberish = "other-gibberish",
///     }
/// }
///
/// assert_eq!(MyEnum::Foo.as_static_str(), "foo");
/// assert_eq!(NoDisplayEnum::ENUM_VARIANTS.len(), 3);
/// ```
#[macro_export]
macro_rules! stringable_enum {
    ($enum:ident { $( $variant:ident = $strval:expr ),+$(,)? }) => {

        $crate::stringable_enum!(@no-display $enum { $( $variant = $strval ),+ });

        impl ::core::fmt::Display for $enum {
            #[inline(always)]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.pad(self.as_static_str())
            }
        }

    };

    (@no-display $enum:ident { $( $variant:ident = $strval:expr ),+$(,)? }) => {
        impl $enum {
            pub const NUM_VARIANTS: usize = *&[$(Self::$variant),+].len();
            pub const ENUM_VARIANTS: [Self; Self::NUM_VARIANTS] = [$(Self::$variant),+];
            pub const ENUM_VARIANT_STRINGS: [&'static str; Self::NUM_VARIANTS] = [$($strval),+];

            pub const fn as_static_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $strval,)+
                }
            }
        }
        impl ::std::str::FromStr for $enum {
            type Err = ();
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    $( $strval => Ok(Self::$variant), )+
                    _ => Err(()),
                }
            }
        }
    };
}

#[cfg(feature = "ansi")]
/// A struct for use in argparsing to control whether ansi colors should be used
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "clap", clap(rename_all = "lowercase", verbatim_doc_comment))]
pub enum Color {
    /// Always use colors
    Always,
    /// Autodetect color support
    #[default]
    Auto,
    /// Never use colors
    Never,
}
#[cfg(feature = "ansi")]
stringable_enum! {
    Color {
        Always = "always",
        Auto = "auto",
        Never = "never",
    }
}
#[cfg(feature = "ansi")]
impl Color {
    /// returns True if the provided stream is a terminal.
    #[inline]
    pub fn detect(&self, stream: &impl std::io::IsTerminal) -> bool {
        match self {
            Self::Always => true,
            Self::Auto => stream.is_terminal(),
            Self::Never => false,
        }
    }
}

#[cfg(feature = "serde")]
/// An enum to allow ser/de of user-configured log levels
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "clap", clap(rename_all = "lowercase", verbatim_doc_comment))]

pub enum LogLevelSerdable {
    Trace,
    #[cfg_attr(debug_assertions, default)]
    Debug,
    Info,
    #[cfg_attr(not(debug_assertions), default)]
    Warn,
    Error,
}

#[cfg(feature = "serde")]
impl LogLevelSerdable {
    pub const fn to_tracing(&self) -> Level {
        match self {
            Self::Trace => Level::TRACE,
            Self::Debug => Level::DEBUG,
            Self::Info => Level::INFO,
            Self::Warn => Level::WARN,
            Self::Error => Level::ERROR,
        }
    }
}

#[cfg(feature = "serde")]
stringable_enum! {
    LogLevelSerdable {
        Trace = "trace",
        Debug = "debug",
        Info = "info",
        Warn = "warn",
        Error = "error",
    }
}
