use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use core::clone::Clone;
use core::cmp::PartialEq;
use core::convert::{AsRef, From};
use core::fmt;
use core::hash::{Hash, Hasher};
use core::iter::{FromIterator, IntoIterator};

/// Opaque Key is a representation of a key.
///
/// It is owned, and largely forms a contract for
/// key to follow.
#[derive(Clone, Eq)]
pub struct Key {
    data: Cow<'static, str>,
}

impl Key {
    /// Returns the length of self.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the length of self.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Take as a `str`
    pub fn as_str(&self) -> &str {
        self.data.as_ref()
    }
}
impl AsRef<str> for Key {
    #[inline]
    fn as_ref(&self) -> &str {
        self.data.as_ref()
    }
}

impl Default for Key {
    fn default() -> Key {
        Key {
            data: Cow::Borrowed(""),
        }
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.data {
            Cow::Borrowed(ref ptr) => ptr.hash(state),
            Cow::Owned(ref ptr) => ptr.hash(state),
        }
    }
}

impl From<&'static str> for Key {
    #[inline(always)]
    fn from(data: &'static str) -> Key {
        Key {
            data: Cow::Borrowed(data),
        }
    }
}

impl From<String> for Key {
    fn from(data: String) -> Key {
        Key {
            data: Cow::Owned(data),
        }
    }
}

impl From<Key> for String {
    fn from(s: Key) -> String {
        s.to_string()
    }
}

impl FromIterator<char> for Key {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Key {
        Key {
            data: Cow::Owned(iter.into_iter().collect::<String>()),
        }
    }
}

impl<'a> FromIterator<&'a char> for Key {
    fn from_iter<I: IntoIterator<Item = &'a char>>(iter: I) -> Key {
        Key {
            data: Cow::Owned(iter.into_iter().collect::<String>()),
        }
    }
}

impl FromIterator<String> for Key {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Key {
        Key {
            data: Cow::Owned(iter.into_iter().collect::<String>()),
        }
    }
}

impl<'a> FromIterator<&'a String> for Key {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> Key {
        Key {
            data: Cow::Owned(
                iter.into_iter().map(|x| x.as_str()).collect::<String>(),
            ),
        }
    }
}

impl<'a> FromIterator<&'a str> for Key {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Key {
        Key {
            data: Cow::Owned(iter.into_iter().collect::<String>()),
        }
    }
}

impl<'a> FromIterator<Cow<'a, str>> for Key {
    fn from_iter<I: IntoIterator<Item = Cow<'a, str>>>(iter: I) -> Key {
        Key {
            data: Cow::Owned(iter.into_iter().collect::<String>()),
        }
    }
}

impl PartialEq<str> for Key {
    fn eq(&self, other: &str) -> bool {
        self.as_ref().eq(other)
    }
}

impl PartialEq<&str> for Key {
    fn eq(&self, other: &&str) -> bool {
        self.as_ref().eq(*other)
    }
}

impl PartialEq<String> for Key {
    fn eq(&self, other: &String) -> bool {
        self.as_ref().eq(other.as_str())
    }
}

impl PartialEq<Self> for Key {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.data {
            Cow::Borrowed(ref ptr) => write!(f, "{}", ptr),
            Cow::Owned(ref ptr) => write!(f, "{}", ptr),
        }
    }
}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.data {
            Cow::Borrowed(ref ptr) => write!(f, "{:?}", ptr),
            Cow::Owned(ref ptr) => write!(f, "{:?}", ptr),
        }
    }
}
