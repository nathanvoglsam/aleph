//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use std::borrow::{Borrow, Cow};
use std::ops::Deref;

pub const SEPARATOR: char = '/';
pub const SEPARATOR_STR: &'static str = "/";
pub const SEPARATOR_BYTE: u8 = *SEPARATOR_STR.as_bytes().first().unwrap();

pub const DOT: char = '.';
pub const DOT_STR: &'static str = ".";
pub const DOT_BYTE: u8 = *DOT_STR.as_bytes().first().unwrap();

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VPathBuf(String);

impl VPathBuf {
    /// Constructs a new, empty path.
    pub const fn new() -> Self {
        Self(String::new())
    }

    /// Constructs a new, empty path.
    ///
    /// Reserves 'capacity' bytes in the internal buffer.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(String::with_capacity(capacity))
    }

    /// Converts this `VPathBuf` into a [boxed](Box) [`VPath`].
    #[must_use = "`self` will be dropped if the result is not used"]
    #[inline]
    pub fn into_boxed_path(self) -> Box<VPath> {
        let rw = Box::into_raw(self.0.into_boxed_str()) as *mut VPath;
        unsafe { Box::from_raw(rw) }
    }

    /// Consumes and leaks the `VPathBuf`, returning a mutable reference to the contents,
    /// `&'a mut VPath`.
    ///
    /// The caller has free choice over the returned lifetime, including 'static.
    /// Indeed, this function is ideally used for data that lives for the remainder of
    /// the program's life, as dropping the returned reference will cause a memory leak.
    ///
    /// It does not reallocate or shrink the `VPathBuf`, so the leaked allocation may include
    /// unused capacity that is not part of the returned slice. If you want to discard excess
    /// capacity, call [`into_boxed_path`], and then [`Box::leak`] instead.
    /// However, keep in mind that trimming the capacity may result in a reallocation and copy.
    ///
    /// [`into_boxed_path`]: Self::into_boxed_path
    #[inline]
    pub fn leak<'a>(self) -> &'a mut VPath {
        VPath::from_inner_mut(self.0.leak())
    }

    pub fn push<P: AsRef<VPath>>(&mut self, path: P) {
        self.__push(path.as_ref())
    }

    /// Internal implementation of [`VPath::push`]. Separated from the generic function exposed
    /// publicly so we monomorph less code.
    fn __push(&mut self, path: &VPath) {
        if path.is_absolute() {
            // If the input path is absolute then we just replace the existing path string in
            // self.
            self.0.clear();
            self.0.push_str(path.to_str());
        } else {
            // Otherwise we need to join the paths somehow...

            // If the existing path in 'self' doesn't have a trailing separator already we need
            // to add one.
            self.push_trailing_sep();

            // And push the input string
            self.0.push_str(path.to_str());
        }
    }

    pub fn push_trailing_sep(&mut self) {
        match self.0.as_bytes().last() {
            Some(&SEPARATOR_BYTE) => {}
            _ => self.0.push(SEPARATOR),
        }
    }

    pub fn pop_trailing_sep(&mut self) {
        match self.0.as_bytes().last() {
            Some(&SEPARATOR_BYTE) => {
                self.0.pop();
            }
            _ => {}
        }
    }

    pub const fn as_path(&self) -> &VPath {
        VPath::from_inner(self.0.as_str())
    }
}

// == DISPLAY TRAITS == //

impl core::fmt::Debug for VPathBuf {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&**self, formatter)
    }
}

impl core::fmt::Display for VPathBuf {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&**self, formatter)
    }
}

// == CONVERSION TRAITS == //

impl<'a> From<&'a VPath> for VPathBuf {
    fn from(value: &'a VPath) -> Self {
        VPathBuf(String::from(&value.0))
    }
}

// == DEREF TRAITS == //

impl Deref for VPathBuf {
    type Target = VPath;

    fn deref(&self) -> &Self::Target {
        self.as_path()
    }
}

// == AS TRAITS == //

impl AsRef<VPath> for VPathBuf {
    fn as_ref(&self) -> &VPath {
        self.as_path()
    }
}

impl AsRef<str> for VPathBuf {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<String> for VPathBuf {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

// == BORROW TRAITS == //

impl Borrow<VPath> for VPathBuf {
    fn borrow(&self) -> &VPath {
        VPath::from_inner(&self.0)
    }
}

// == COMPARISON TRAITS == //

impl PartialEq<String> for VPathBuf {
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<str> for VPathBuf {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

/// A slice of a path (akin to [`str`]).
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VPath(str);

impl VPath {
    const fn from_inner(v: &str) -> &Self {
        unsafe { &*(v as *const str as *const VPath) }
    }

    const fn from_inner_mut(v: &mut str) -> &mut Self {
        unsafe { &mut *(v as *mut str as *mut VPath) }
    }

    /// Constructs a new `VPath` from the provided string.
    pub const fn new(v: &str) -> &Self {
        Self::from_inner(v)
    }

    /// Yields a [`&str`] slice of the `VPath`.
    pub const fn to_str(&self) -> &str {
        &self.0
    }

    /// Checks whether the `VPath` is empty.
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the byte length of the `VPath`.
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Converts a `VPath` to an owned [`VPathBuf`].
    ///
    /// # Examples
    ///
    /// ```
    /// use aleph_engine::vpath::{VPath, VPathBuf};
    ///
    /// let path_buf = VPath::new("foo.txt").to_path_buf();
    /// assert_eq!(path_buf, VPathBuf::from("foo.txt"));
    /// ```
    pub fn to_path_buf(&self) -> VPathBuf {
        VPathBuf::from(self)
    }

    /// Returns `true` if the `VPath` is absolute, i.e., if it is independent of
    /// the current directory.
    ///
    /// A `VPath` is considered absolute if it starts with the root '/'.
    pub const fn is_absolute(&self) -> bool {
        // A very roundabout way of checking if the first character is '/', but we do it this way
        // so the function can be const.
        match (SEPARATOR_STR.as_bytes().first(), self.0.as_bytes().first()) {
            (Some(root), Some(first)) => *root == *first,
            (Some(_), None) => false,
            (None, _) => unreachable!(),
        }
    }

    /// Returns `true` if the `VPath` is relative, i.e., not absolute.
    ///
    /// See [`VPath::is_absolute`]'s documentation for more details.
    pub const fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    /// Checks whether the path ends in a trailing separator.
    ///
    /// This is generally done to ensure that a path is treated as a directory, not a file,
    /// although it does not actually guarantee that such a path is a directory on the underlying
    /// file system.
    ///
    /// Despite this behavior, two paths are still considered the same whether they have a
    /// trailing separator or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use aleph_engine::vpath::VPath;
    ///
    /// assert!(VPath::new("dir/").has_trailing_sep());
    /// assert!(VPath::new("dir//").has_trailing_sep());
    /// assert!(!VPath::new("file.rs").has_trailing_sep());
    /// ```
    pub const fn has_trailing_sep(&self) -> bool {
        matches!(self.0.as_bytes().last(), Some(&SEPARATOR_BYTE))
    }

    /// Produces an iterator over `VPath` and its ancestors.
    ///
    /// The iterator will yield the `VPath` that is returned if the [`parent`] method is used zero
    /// or more times. If the [`parent`] method returns [`None`], the iterator will do likewise.
    /// The iterator will always yield at least one value, namely `Some(&self)`. Next it will yield
    /// `&self.parent()`, `&self.parent().and_then(Path::parent)` and so on.
    ///
    /// # Examples
    ///
    /// ```
    /// use aleph_engine::vpath::VPath;
    ///
    /// let mut ancestors = VPath::new("/foo/bar").ancestors();
    /// assert_eq!(ancestors.next(), Some(VPath::new("/foo/bar")));
    /// assert_eq!(ancestors.next(), Some(VPath::new("/foo")));
    /// assert_eq!(ancestors.next(), Some(VPath::new("/")));
    /// assert_eq!(ancestors.next(), None);
    ///
    /// let mut ancestors = VPath::new("../foo/bar").ancestors();
    /// assert_eq!(ancestors.next(), Some(VPath::new("../foo/bar")));
    /// assert_eq!(ancestors.next(), Some(VPath::new("../foo")));
    /// assert_eq!(ancestors.next(), Some(VPath::new("..")));
    /// assert_eq!(ancestors.next(), Some(VPath::new("")));
    /// assert_eq!(ancestors.next(), None);
    /// ```
    ///
    /// [`parent`]: Path::parent
    #[inline]
    pub fn ancestors(&self) -> Ancestors<'_> {
        Ancestors { next: Some(&self) }
    }

    pub fn components(&self) -> Components<'_> {
        if self.is_empty() {
            Components {
                yield_root: false,
                next: None,
            }
        } else {
            Components {
                yield_root: self.is_absolute(),
                next: Some(&self.0),
            }
        }
    }

    pub fn reverse_components(&self) -> ReverseComponents<'_> {
        if self.is_empty() {
            ReverseComponents { next: None }
        } else {
            ReverseComponents {
                next: Some(&self.0),
            }
        }
    }

    /// Returns the `VPath` without its final component, if there is one.
    ///
    /// This means it returns `Some("")` for relative paths with one component.
    ///
    /// Returns [`None`] if the path terminates in a root, or if it's the empty string.
    ///
    /// # Examples
    /// ```
    /// use aleph_engine::vpath::VPath;
    ///
    /// let path = VPath::new("/foo/bar");
    /// let parent = path.parent().unwrap();
    /// assert_eq!(parent, VPath::new("/foo"));
    ///
    /// let grand_parent = parent.parent().unwrap();
    /// assert_eq!(grand_parent, VPath::new("/"));
    /// assert_eq!(grand_parent.parent(), None);
    ///
    /// let relative_path = VPath::new("foo/bar");
    /// let parent = relative_path.parent();
    /// assert_eq!(parent, Some(VPath::new("foo")));
    /// let grand_parent = parent.and_then(VPath::parent);
    /// assert_eq!(grand_parent, Some(VPath::new("")));
    /// let great_grand_parent = grand_parent.and_then(VPath::parent);
    /// assert_eq!(great_grand_parent, None);
    /// ```
    #[must_use]
    pub fn parent(&self) -> Option<&VPath> {
        // Handle the empty case explicitly to simplify handling other cases.
        if self.is_empty() {
            return None;
        }

        let mut parent = self.reverse_components();
        match parent.next() {
            None => None,
            Some(_) => Some(parent.as_path()),
        }
    }

    /// Trims a trailing separator from a path, if possible.
    ///
    /// The resulting path will return false for [`has_trailing_sep`](Self::has_trailing_sep) for
    /// most paths.
    ///
    /// Some paths, like `/`, cannot be trimmed in this way.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use aleph_engine::vpath::VPath;
    ///
    /// assert_eq!(VPath::new("dir//").trim_trailing_sep().to_str(), "dir");
    /// assert_eq!(VPath::new("dir/").trim_trailing_sep().to_str(), "dir");
    /// assert_eq!(VPath::new("dir").trim_trailing_sep().to_str(), "dir");
    /// assert_eq!(VPath::new("/").trim_trailing_sep().to_str(), "/");
    /// assert_eq!(VPath::new("//").trim_trailing_sep().to_str(), "//");
    /// ```
    #[must_use]
    #[inline]
    pub fn trim_trailing_sep(&self) -> &VPath {
        if self.has_trailing_sep() && (!self.is_absolute() || self.parent().is_some()) {
            let mut bytes = self.0.as_bytes();
            while let Some((&SEPARATOR_BYTE, init)) = bytes.split_last() {
                bytes = init;
            }

            // SAFETY: Trimming trailing ASCII bytes will retain the validity of the string.
            VPath::new(unsafe { str::from_utf8_unchecked(bytes) })
        } else {
            self
        }
    }
}

// == DISPLAY TRAITS == //

impl core::fmt::Debug for VPath {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.0, formatter)
    }
}

impl core::fmt::Display for VPath {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, formatter)
    }
}

// == CONVERSION TRAITS == //

// == DEREF TRAITS == //

// == AS TRAITS == //

impl AsRef<str> for VPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// == BORROW TRAITS == //

impl ToOwned for VPath {
    type Owned = VPathBuf;

    fn to_owned(&self) -> Self::Owned {
        VPathBuf::from(self)
    }
}

// == COMPARISON TRAITS == //

impl PartialEq<String> for VPath {
    fn eq(&self, other: &String) -> bool {
        <str as PartialEq>::eq(&self.0, other)
    }
}

impl PartialEq<str> for VPath {
    fn eq(&self, other: &str) -> bool {
        <str as PartialEq>::eq(&self.0, other)
    }
}

// == EXTRA CROSS TYPE COMPARISON TRAITS == //

macro_rules! impl_cmp {
    ($lhs:ty, $rhs: ty) => {
        impl PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <VPath as PartialEq>::eq(self, other)
            }
        }

        impl PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <VPath as PartialEq>::eq(self, other)
            }
        }

        impl PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<::std::cmp::Ordering> {
                <VPath as PartialOrd>::partial_cmp(self, other)
            }
        }

        impl PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<::std::cmp::Ordering> {
                <VPath as PartialOrd>::partial_cmp(self, other)
            }
        }
    };
}

impl_cmp!(VPathBuf, VPath);
impl_cmp!(VPathBuf, &VPath);
impl_cmp!(Cow<'_, VPath>, VPath);
impl_cmp!(Cow<'_, VPath>, &VPath);
impl_cmp!(Cow<'_, VPath>, VPathBuf);

/// An iterator over [`VPath`] and its ancestors.
///
/// This `struct` is created by the [`ancestors`] method on [`VPath`].
/// See its documentation for more.
///
/// # Examples
///
/// ```
/// use aleph_engine::vpath::VPath;
///
/// let path = VPath::new("/foo/bar");
///
/// for ancestor in path.ancestors() {
///     println!("{}", ancestor.display());
/// }
/// ```
///
/// [`ancestors`]: VPath::ancestors
#[derive(Copy, Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Ancestors<'a> {
    next: Option<&'a VPath>,
}

impl<'a> Iterator for Ancestors<'a> {
    type Item = &'a VPath;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        self.next = next.and_then(VPath::parent);
        next
    }
}

impl std::iter::FusedIterator for Ancestors<'_> {}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Component<'a> {
    Root,
    Segment(&'a str),
}

/// An iterator over the [`Component`]s of a [`VPath`].
///
/// This `struct` is created by the [`components`] method on [`VPath`].
/// See its documentation for more.
///
/// # Examples
///
/// ```
/// use aleph_engine::vpath::VPath;
///
/// let path = VPath::new("/tmp/foo/bar.txt");
///
/// for component in path.components() {
///     println!("{component:?}");
/// }
/// ```
///
/// [`components`]: VPath::components
#[derive(Copy, Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Components<'a> {
    yield_root: bool,
    next: Option<&'a str>,
}

impl<'a> Components<'a> {
    pub fn as_path(&self) -> &'a VPath {
        match self.next {
            None => VPath::new(""),
            Some(path) => VPath::new(path),
        }
    }
}

impl<'a> Iterator for Components<'a> {
    type Item = Component<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.next {
                // The terminal state, always just yield none here. There's nothing more to do.
                None => None,

                // Otherwise we try and split the string in two at the first path separator.
                Some(next) => match next.split_once(SEPARATOR) {
                    None => {
                        // If we reach here then we failed to split the path, meaning there's no
                        // more path separators (or the path is empty). This is the last segment
                        // to handle before transitioning to the terminal state.

                        // Move to terminal state.
                        self.next = None;

                        // If there's a trailing separator we may have an empty trailing segment.
                        // We don't yield these from the iterator and return None instead as they
                        // have no meaning.
                        if next.is_empty() {
                            None
                        } else {
                            Some(Component::Segment(next))
                        }
                    }
                    Some((component, rest)) => {
                        // If we reach here then we have split the path into two pieces, a prefix
                        // 'component' that contains no separators and a trailing remainder that may
                        // contain 0 or more additional separators. 'rest' may also be empty, but
                        // that case is handled elsewhere.
                        //
                        // 'component' contains the path segment we want to try and yield. It is
                        // possible for 'component' to be empty, say if an input path contains
                        // multiple separators like 'this/is//a/bad/path'. These are normalized away
                        // by this iterator.

                        // Advance the iterator
                        self.next = Some(rest);

                        if component.is_empty() {
                            // If the component is empty then we _don't_ yield 'component' and
                            // instead look for following non-empty segment. Because we just
                            // advanced the iterator we can just bail to the start of the loop
                            // again, this effectively skips the empty segment.
                            //
                            // Except if 'yield_root' is true, which must only be set for paths
                            // that start with a leading separator (absolute). In that case we
                            // yield the 'root' component instead as we just processed the root
                            // separator.
                            if self.yield_root {
                                // We can only yield one root.
                                self.yield_root = false;
                                Some(Component::Root)
                            } else {
                                // Otherwise we skip the empty segment
                                continue;
                            }
                        } else {
                            // Good new! we've got a valid path segment.
                            Some(Component::Segment(component))
                        }
                    }
                },
            };
        }
    }
}

impl std::iter::FusedIterator for Components<'_> {}

/// An iterator over the [`Component`]s of a [`VPath`], from right-to-left.
///
/// This `struct` is created by the [`components`] method on [`VPath`].
/// See its documentation for more.
///
/// # Examples
///
/// ```
/// use aleph_engine::vpath::VPath;
///
/// let path = VPath::new("/tmp/foo/bar.txt");
///
/// for component in path.components() {
///     println!("{component:?}");
/// }
/// ```
///
/// [`components`]: VPath::components
#[derive(Copy, Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ReverseComponents<'a> {
    next: Option<&'a str>,
}

impl<'a> ReverseComponents<'a> {
    pub fn as_path(&self) -> &'a VPath {
        match self.next {
            None => VPath::new(""),
            Some(path) => VPath::new(path),
        }
    }
}

impl<'a> Iterator for ReverseComponents<'a> {
    type Item = Component<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.next {
                // The terminal state, always just yield none here. There's nothing more to do.
                None => None,

                // Otherwise we try and split the string in two at the first path separator.
                Some(next) => match next.rsplit_once(SEPARATOR) {
                    None => {
                        // Move to terminal state.
                        self.next = None;

                        if next.is_empty() {
                            Some(Component::Root)
                        } else {
                            Some(Component::Segment(next))
                        }
                    }
                    Some((rest, component)) => {
                        // Advance the iterator
                        self.next = Some(rest);

                        if component.is_empty() {
                            continue;
                        } else {
                            Some(Component::Segment(next))
                        }
                    }
                },
            };
        }
    }
}

impl std::iter::FusedIterator for ReverseComponents<'_> {}
