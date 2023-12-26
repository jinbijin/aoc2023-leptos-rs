#[cfg(feature = "ssr")]
mod graph;

#[cfg(feature = "ssr")]
pub use graph::{AdjacencyHashGraph, CycleResult, DistanceHashGraph};

#[macro_export]
macro_rules! create_formatted_flat_enum {
    ($type_name:ident error $error_type_name:ident with [$( $entry_name:ident => $value:expr ),*]) => {
        #[derive(Clone)]
        pub struct $error_type_name(String);

        impl std::fmt::Display for $error_type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "invalid value \"{}\" for {}", &self.0, stringify!($type_name))
            }
        }

        impl std::fmt::Debug for $error_type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(self, f)
            }
        }

        impl std::error::Error for $error_type_name { }

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $type_name {
            $(
                $entry_name,
            )*
        }

        impl std::fmt::Debug for $type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        $type_name::$entry_name => write!(f, "{}", $value),
                    )*
                }
            }
        }

        impl std::str::FromStr for $type_name {
            type Err = $error_type_name;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $value => Ok($type_name::$entry_name),
                    )*
                    _ => Err($error_type_name(s.to_string()))
                }
            }
        }
    };
}