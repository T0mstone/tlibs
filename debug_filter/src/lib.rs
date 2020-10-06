//! # Debug Filters
//!
//! This crate provides a more sophisticated alternative to the common debug level pattern.
//!
//! The core item are filters. A filter is a string which is divided into components.
//! A filter is similar to a file path. (1)
//!
//! When you turn allow the filter `a`, this allows all filters of the form `a`, `a.b`, `a.b.c`, ...
//!
//! You can also allow a filter like `a.b.c`, which allows `a.b.c`, `a.b.c.d`, ...
//!
//! ## Negative filters
//!
//! There are also negative filters, which disallow the specified filter and all its children. (2)
//!
//! For example `!a.b` disallows `a.b`, `a.b.c`, ...
//!
//! ## Universal filter
//!
//! There exists a universal filter that allows everything. It is generic, like (1) and (2), and the default is `"all"`
//!
//! ## Multiple filters
//!
//! You can specify multiple filters at the same time. When two filters apply, the more specific one is chosen.
//!
//! Example: specifying `a.b`, `!a.b.c` and `a.b.c.d` will allow `a.b`, `a.b.c.d`, `a.b.e` (only if `e != c`), `a.b.e.f`, ... and disallow `a.b.c`, `a.b.c.g` (only if `g != d`), `a.b.c.g.h`, ...
//!
//! Specifying multiple filters in one string (3) requires another separator, which too is generic and whose default is `':'`.
//! The filters from the example would then be `a.b:!a.b.c:a.b.c.d`
//!
//! Every filter not listed will be assumed to be disallowed. Include the universal filter in the list to change that.
//!
//! ### Footnotes
//! 1. This crate is generic over the separator for components. The default is `'.'` so it is used in the examples here.
//! 2. This crate is generic over the prefix for negating components. The default is `'!'` so it is used in the examples here.
//! 3. This is done by the [`parse`](struct.Filters.html#method.parse) method

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::iter::once;
use std::marker::PhantomData;

/// This is a workaround for const generics. This trait assigns a type some constant value.
pub trait ConstValue<T: Copy> {
    /// The assigned value
    const VALUE: T;
}

/// This macro allows quick declaration of a type for [`ConstValue`](trait.ConstValue.html)
///
/// # Syntax
/// ```
/// # use debug_filter::const_types;
/// # #[derive(Copy, Clone)] struct Type;
/// # const CONST: Type = Type;
/// const_types! {
///     #[derive(Debug)]
///     pub const type Name1: Type = CONST;
///     /// Documentation
///     const type Name2: u8 = 17;
/// }
/// ```
#[macro_export]
macro_rules! const_types {
    ($($(#[$m:meta])* $v:vis const type $name:ident: $t:ty = $char:expr);*$(;)?) => {
        $(
            $(#[$m])*
            #[derive(Copy, Clone)]
            $v enum $name {}
            impl $crate::ConstValue<$t> for $name {
                const VALUE: $t = $char;
            }
    	)*
    };
}

const_types! {
    /// A const type for the character U+002E Full Stop '.'
    pub const type U002E: char = '.';
    /// A const type for the character U+003A Colon ':'
    pub const type U003A: char = ':';
    /// A const type for the character U+0021 Exclamation Mark '!'
    pub const type U0021: char = '!';
    /// A const type for the string "all"
    pub const type StrAll: &'static str = "all";
}
#[allow(missing_docs)]
pub type FullStop = U002E;
#[allow(missing_docs)]
pub type Point = U002E;
#[allow(missing_docs)]
pub type Colon = U003A;
#[allow(missing_docs)]
pub type ExclamationMark = U0021;

/// A trait that includes all the parameters you can customize the parsing behaviour of `Filters` with at compile time
pub trait FilterParser {
    /// The separator for filter components
    type HierarchySeparator: ConstValue<char>;
    /// The separator for different filters when parsing a literal
    type FilterListSeparator: ConstValue<char>;
    /// The prefix for a filter that inverts it
    type NegativeFilterPrefix: ConstValue<char>;
    /// The name of the universal filter
    type UniversalFilter: ConstValue<&'static str>;
}

/// A macro to quickly declare a [`FilterParser`](trait.FilterParser.html)
///
/// # Syntax
/// ```
/// # use debug_filter::*;
/// # const_types!(pub const type A: char = 'a'; pub const type B: char = 'b'; pub const type C: char = 'c'; pub const type D: &'static str = "d");
/// declare_filter_parser! {
///     pub const struct Name {
///         type HierarchySeparator = A;
///         type FilterListSeparator = B;
///         type NegativeFilterPrefix = C;
///         type UniversalFilter = D;
///     }
/// }
/// ```
#[macro_export]
macro_rules! declare_filter_parser {
    ($(#[$m:meta])* $v:vis const struct $name:ident {
        $(type $t:ident = $val:ty);*$(;)?
    }) => {
        $(#[$m])*
        $v enum $name {}
        impl $crate::FilterParser for $name {
            $(type $t = $val;)*
        }
    };
}
declare_filter_parser! {
    #[allow(missing_docs)]
    pub const struct DefaultFilterParser {
        type HierarchySeparator = Point;
        type FilterListSeparator = Colon;
        type NegativeFilterPrefix = ExclamationMark;
        type UniversalFilter = StrAll;
    }
}

/// A type that stores the filters that are currently active
pub struct Filters<'a, P: FilterParser = DefaultFilterParser> {
    /// A map of `filter -> allowed`
    pub filters: HashMap<&'a str, bool>,
    /// The `allowed` value for all filters not in `self.filters`
    pub default: bool,
    _marker: PhantomData<P>,
}

impl<'a, P: FilterParser> fmt::Debug for Filters<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Filters")
            .field("filters", &self.filters)
            .field("default", &self.default)
            .finish()
    }
}

impl<'a, P: FilterParser> Default for Filters<'a, P> {
    fn default() -> Self {
        Self {
            filters: HashMap::default(),
            default: false,
            _marker: PhantomData,
        }
    }
}

impl<'a, P: FilterParser> PartialEq for Filters<'a, P> {
    fn eq(&self, other: &Self) -> bool {
        self.filters == other.filters && self.default == other.default
    }
}
impl<'a, P: FilterParser> Eq for Filters<'a, P> {}

/// Like [`Filters`](struct.Filters.html), but owns the data its keys are from
pub struct OwningFilters<P: FilterParser = DefaultFilterParser> {
    /// A map of `filter -> allowed`
    pub filters: HashMap<*const str, bool>,
    /// The `allowed` value for all filters not in `self.filters`
    pub default: bool,
    _container: String,
    _marker: PhantomData<P>,
}

impl<P: FilterParser> OwningFilters<P> {
    /// Get `Filters` with the data from `self`
    // SAFETY: since the returned lifetime for the keys is the same as the lifetime for &self, this is safe
    pub fn as_filters(&self) -> Filters<P> {
        Filters {
            filters: self
                .filters
                .clone()
                .into_iter()
                .map(|(k, v)| (unsafe { &*k }, v))
                .collect(),
            default: self.default,
            _marker: PhantomData,
        }
    }
}

impl<'a, P: FilterParser> Filters<'a, P> {
    #[allow(missing_docs)]
    #[inline]
    pub fn new(filters: HashMap<&'a str, bool>, default: bool) -> Self {
        Self {
            filters,
            default,
            _marker: PhantomData,
        }
    }

    #[allow(clippy::needless_lifetimes)] // note: this is a clippy bug - the lifetime is needed for the where clause
    /// gets the state of the provided filter
    pub fn check(&self, s: &'a str) -> bool {
        s.char_indices()
            .filter(|t| t.1 == P::HierarchySeparator::VALUE)
            .map(|t| &s[..t.0])
            .chain(once(s))
            .filter_map(|s| self.filters.get(s).copied())
            .next_back()
            .unwrap_or(self.default)
    }
}

impl<'a, P: FilterParser> Filters<'a, P> {
    /// parse a list of filters
    pub fn parse_each(mut hs: HashSet<&'a str>) -> Self {
        let default = hs.remove(P::UniversalFilter::VALUE);
        let mut filters = HashMap::new();
        for s in hs {
            if s.starts_with(P::NegativeFilterPrefix::VALUE) {
                let s = &s[1..];
                match filters.entry(s) {
                    // just ref'ing this arm is shorter than having to do `let _ = ` on both
                    Entry::Occupied(e) => &mut e.remove(),
                    Entry::Vacant(e) => e.insert(false),
                };
            } else if filters
                .contains_key(format!("{}{}", P::NegativeFilterPrefix::VALUE, s).as_str())
            {
                let _ = filters.remove(&s);
            } else {
                filters.insert(s, true);
            }
        }
        Self::new(filters, default)
    }

    /// parse a single string into a list of filters
    /// # Example
    /// This example reads the `debug_filters` environment variable at compile time -
    /// if you want to read it at runtime, use [`from_env`](#method.from_env) or [`from_option_env`](#method.from_option_env)
    /// ```
    /// # use debug_filter::Filters;
    /// # use std::env;
    /// use once_cell::sync::Lazy;
    /// static FILTERS: Lazy<Filters> = Lazy::new(|| Filters::parse(env!("debug_filters")));
    /// ```
    #[inline]
    pub fn parse(s: &'a str) -> Self {
        let hs: HashSet<_> = s.split(P::FilterListSeparator::VALUE).collect();
        Self::parse_each(hs)
    }

    #[inline]
    fn parse_string(s: String) -> OwningFilters<P> {
        let r = Filters::<'_, P>::parse(&s);
        let filters = r
            .filters
            .into_iter()
            .map(|(k, v)| (k as *const str, v))
            .collect();
        let default = r.default;
        OwningFilters {
            filters,
            default,
            _container: s,
            _marker: PhantomData,
        }
    }

    /// parse the content of the environment variable `name` at runtime and panic if it is not present or not unicode
    #[inline]
    pub fn from_env(name: &str) -> OwningFilters<P> {
        let s = std::env::var(name).unwrap_or_default();
        Self::parse_string(s)
    }

    /// parse the content of the environment variable `name` at runtime. Returns None that it is not present or not unicode
    #[inline]
    pub fn from_option_env(name: &str) -> Option<OwningFilters<P>> {
        let s = std::env::var(name).ok()?;
        Some(Self::parse_string(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filters() -> Filters<'static> {
        let mut hm = HashMap::new();
        hm.insert("a", true);
        hm.insert("b", true);
        hm.insert("a.b", false);
        hm.insert("a.b.c", true);

        Filters::new(hm, false)
    }

    fn filters2() -> Filters<'static> {
        let mut hm = HashMap::new();
        hm.insert("a.b", false);

        Filters::new(hm, true)
    }

    #[test]
    fn check_fn() {
        let f = filters();

        assert!(f.check(&"a"));
        assert!(f.check(&"b"));
        assert!(!f.check(&"a.b"));
        assert!(f.check(&"a.c"));
        assert!(f.check(&"a.b.c"));
        assert!(f.check(&"a.b.c.d"));
        assert!(!f.check(&"a.b.d"));

        let f2 = filters2();
        assert!(!f2.check(&"a.b"));
    }

    #[test]
    fn parse_each() {
        let mut hm = HashSet::new();
        hm.insert("a");
        hm.insert("b");
        hm.insert("!a.b");
        hm.insert("a.b.c");

        let f = Filters::parse_each(hm);

        assert_eq!(f, filters());

        let mut hm2 = HashSet::new();
        hm2.insert("all");
        hm2.insert("!a.b");

        let f2 = Filters::parse_each(hm2);

        assert_eq!(f2, filters2());
    }

    #[test]
    fn parse() {
        let f = Filters::parse("a:b:!a.b:a.b.c");
        assert_eq!(f, filters());
        let f2 = Filters::parse("all:!a.b");
        assert_eq!(f2, filters2());
    }
}
