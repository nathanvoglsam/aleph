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

/// An owned version of [`VPath`]. See `VPath` docs for more info.
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

    /// Extends `self` with `path`.
    ///
    /// If `path` is absolute, it replaces the current path.
    ///
    /// Consider using [`VPath::join`] if you need a new `PathBuf` instead of
    /// using this function on a cloned `PathBuf`.
    ///
    /// # Examples
    ///
    /// Pushing a relative path extends the existing path:
    ///
    /// ```
    /// use aleph_engine::vpath::VPathBuf;
    ///
    /// let mut path = VPathBuf::from("/tmp");
    /// path.push("file.bk");
    /// assert_eq!(path, VPathBuf::from("/tmp/file.bk"));
    /// ```
    ///
    /// Pushing an absolute path replaces the existing path:
    ///
    /// ```
    /// use aleph_engine::vpath::VPathBuf;
    ///
    /// let mut path = VPathBuf::from("/tmp");
    /// path.push("/etc");
    /// assert_eq!(path, VPathBuf::from("/etc"));
    /// ```
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
            match self.0.as_bytes().last() {
                Some(&SEPARATOR_BYTE) => {}
                _ => self.0.push(SEPARATOR),
            }

            // And push the input string
            self.0.push_str(path.to_str());
        }
    }

    /// Similar to [`VPath::trim_trailing_sep`], but mutates the underlying path instead.
    pub fn pop_trailing_sep(&mut self) {
        while let Some(&SEPARATOR_BYTE) = self.0.as_bytes().last() {
            self.0.pop();
        }
    }

    /// Get the inner path as a [`VPath`].
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

impl<'a> From<&'a str> for VPathBuf {
    fn from(value: &'a str) -> Self {
        VPathBuf(String::from(value))
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
///
/// `VPath` wraps a `str` into a container that is expected to contain a path. It exposes a similar
/// interface to [`Path`].
///
/// `VPath` however is not concerned with OS path semantics. It adopts unix path semantics with
/// small differences.
///
/// - `.` is not special, and does not mean 'self'.
/// - `..` is not special, and does not mean 'parent'.
/// - `/` is the path separator.
/// - All paths starting with `/` are absolute.
/// - All paths without a leading `/` are relative.
///
/// The expected use is virtual-file-system paths. The semantics are application defined and these
/// paths should not be used with the OS filesystem.
///
/// [`Path`]: std::path::Path
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

    /// Produces an iterator over the [`Component`]s of the path.
    ///
    /// When parsing the path, there is a small amount of normalization:
    ///
    /// * Repeated separators are ignored, so `a/b` and `a//b` both have
    ///   `a` and `b` as components.
    ///
    /// * Trailing separators are normalized away, so `/a/b` and `/a/b/` are equivalent.
    ///
    /// # Examples
    ///
    /// ```
    /// use aleph_engine::vpath::VPath;
    /// use aleph_engine::vpath::Component;
    ///
    /// let mut components = VPath::new("/tmp/foo.txt").components();
    ///
    /// assert_eq!(components.next(), Some(Component::Root));
    /// assert_eq!(components.next(), Some(Component::Segment("tmp")));
    /// assert_eq!(components.next(), Some(Component::Segment("foo.txt")));
    /// assert_eq!(components.next(), None)
    /// ```
    pub fn components(&self) -> Components<'_> {
        Components { next: &self.0 }
    }

    /// Produces an iterator over the [`Component`]s of the path in reverse order.
    ///
    /// When parsing the path, there is a small amount of normalization:
    ///
    /// * Repeated separators are ignored, so `a/b` and `a//b` both have
    ///   `a` and `b` as components.
    ///
    /// * Trailing separators are normalized away, so `/a/b` and `/a/b/` are equivalent.
    ///
    /// # Examples
    ///
    /// ```
    /// use aleph_engine::vpath::VPath;
    /// use aleph_engine::vpath::Component;
    ///
    /// let mut components = VPath::new("/tmp/foo.txt").reverse_components();
    ///
    /// assert_eq!(components.next(), Some(Component::Segment("foo.txt")));
    /// assert_eq!(components.next(), Some(Component::Segment("tmp")));
    /// assert_eq!(components.next(), Some(Component::Root));
    /// assert_eq!(components.next(), None)
    /// ```
    pub fn reverse_components(&self) -> ReverseComponents<'_> {
        ReverseComponents { next: &self.0 }
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

        let mut components = self.reverse_components();
        match components.next() {
            Some(Component::Segment(_)) => Some(components.as_path()),
            Some(Component::Root) => None,
            None => None,
        }
    }

    /// Trims a trailing separator from a path, if possible.
    ///
    /// The resulting path will return false for [`VPath::has_trailing_sep`] for most paths.
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
        let bytes = self.0.as_bytes();

        // Preserve root-like paths that are made entirely of separators ("/", "//", ...).
        let Some(last_non_sep) = bytes.iter().rposition(|&v| v != SEPARATOR_BYTE) else {
            return self;
        };

        let end = last_non_sep + 1;
        if end == bytes.len() {
            self
        } else {
            // SAFETY: `end` points to the byte after a non-separator ASCII byte, so this is a
            // valid UTF-8 boundary and the subslice remains valid UTF-8.
            VPath::new(unsafe { str::from_utf8_unchecked(&bytes[..end]) })
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

impl AsRef<VPath> for str {
    fn as_ref(&self) -> &VPath {
        VPath::from_inner(self)
    }
}

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
///     println!("{}", ancestor);
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
    next: &'a str,
}

impl<'a> Components<'a> {
    pub const fn as_path(&self) -> &'a VPath {
        VPath::new(self.next)
    }
}

impl<'a> Iterator for Components<'a> {
    type Item = Component<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.next;
            return match next.split_once(SEPARATOR) {
                None => {
                    // If we reach here then we failed to split the path, meaning there's no
                    // more path separators (or the path is empty). This is the last segment
                    // to handle before transitioning to the terminal state.

                    // Move to terminal state.
                    self.next = "";

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
                    // contain 0 or more additional separators. 'rest' may also be empty.

                    // We found one separator, but there may be more. We proactively strip them
                    // when incrementing the iterator for two reasons.
                    //
                    // - as_path() will return a correct path for the current state of the
                    //   iterator.
                    // - guarantees only the first call to next() will see leading separators.
                    //
                    // This means we need no additional state to determine if we're handling the
                    // root segment.
                    let mut next = rest;
                    loop {
                        match next.split_once(SEPARATOR) {
                            Some((component, rest)) if component.is_empty() => {
                                next = rest;
                                continue;
                            }
                            Some((component, rest)) if component.is_empty() && rest.is_empty() => {
                                break;
                            }
                            Some((_, _)) => {
                                break;
                            }
                            None => {
                                break;
                            }
                        }
                    }

                    // Advance the iterator
                    self.next = next;

                    // 'component' contains the path segment we want to try and yield. If
                    // component is empty then that means we are processing a root segment.
                    // Because we strip any leading separators when advancing the iterator it
                    // is only possible to observe an empty component when processing 1+ leading
                    // separators on the fist call to 'next'.
                    if component.is_empty() {
                        Some(Component::Root)
                    } else {
                        // Good news! we've got a valid path segment.
                        Some(Component::Segment(component))
                    }
                }
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
    next: &'a str,
}

impl<'a> ReverseComponents<'a> {
    pub const fn as_path(&self) -> &'a VPath {
        VPath::new(self.next)
    }
}

impl<'a> Iterator for ReverseComponents<'a> {
    type Item = Component<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        match next.rsplit_once(SEPARATOR) {
            // We've just proven that the path contains exactly a single separator, which
            // is the root path.
            Some((lhs, rhs)) if lhs.is_empty() && rhs.is_empty() => {
                self.next = "";
                Some(Component::Root)
            }

            // We've found that we have a path with a separator that wasn't trailing as rhs
            // is not empty. We've also determined it was a singular leading separator so
            // we should set up to yield the root segment next.
            Some((lhs, rhs)) if lhs.is_empty() => {
                self.next = "/";
                Some(Component::Segment(rhs))
            }

            // We've found a trailing separator. There may be more so we need to trim them
            // all before we can find the true leaf segment we want to yield.
            Some((lhs, rhs)) if rhs.is_empty() => {
                let mut next = lhs;
                let leaf = loop {
                    match next.rsplit_once(SEPARATOR) {
                        Some((lhs, rhs)) if lhs.is_empty() && rhs.is_empty() => {
                            next = "";
                            break Component::Root;
                        }
                        Some((lhs, rhs)) if lhs.is_empty() => {
                            next = "/";
                            break Component::Segment(rhs);
                        }
                        Some((lhs, rhs)) if rhs.is_empty() => {
                            next = lhs;
                            continue;
                        }
                        Some((lhs, rhs)) => {
                            next = lhs;
                            break Component::Segment(rhs);
                        }
                        None => {
                            let out = next;
                            next = "";
                            break Component::Segment(out);
                        }
                    }
                };

                self.next = next;

                loop {
                    match self.next.rsplit_once(SEPARATOR) {
                        Some((lhs, rhs)) if lhs.is_empty() && rhs.is_empty() => {
                            break;
                        }
                        Some((lhs, _rhs)) if lhs.is_empty() => {
                            break;
                        }
                        Some((lhs, rhs)) if rhs.is_empty() => {
                            self.next = lhs;
                            continue;
                        }
                        Some((_lhs, _rhs)) => {
                            break;
                        }
                        None => {
                            break;
                        }
                    }
                }

                Some(leaf)
            }

            // We've found a separator. It wasn't
            Some((lhs, rhs)) => {
                self.next = lhs;

                loop {
                    match self.next.rsplit_once(SEPARATOR) {
                        Some((lhs, rhs)) if lhs.is_empty() && rhs.is_empty() => {
                            break;
                        }
                        Some((lhs, _rhs)) if lhs.is_empty() => {
                            break;
                        }
                        Some((lhs, rhs)) if rhs.is_empty() => {
                            self.next = lhs;
                            continue;
                        }
                        Some((_lhs, _rhs)) => {
                            break;
                        }
                        None => {
                            break;
                        }
                    }
                }

                Some(Component::Segment(rhs))
            }

            // If the path segment in 'next' contains no separators at all then it means we
            // either have the empty path, or a single remaining segment to yield.
            None => {
                // Update the iterator so we don't try to split next time.
                self.next = "";

                // Coerce the empty path
                if next.is_empty() {
                    None
                } else {
                    Some(Component::Segment(next))
                }
            }
        }
    }
}

impl std::iter::FusedIterator for ReverseComponents<'_> {}

#[cfg(test)]
mod tests {
    #[test]
    pub fn reverse_components_test() {
        use super::*;

        let mut components = VPath::new("/tmp/foo.txt").reverse_components();
        assert_eq!(components.as_path(), "/tmp/foo.txt");
        assert_eq!(components.next(), Some(Component::Segment("foo.txt")));
        assert_eq!(components.as_path(), "/tmp");
        assert_eq!(components.next(), Some(Component::Segment("tmp")));
        assert_eq!(components.as_path(), "/");
        assert_eq!(components.next(), Some(Component::Root));
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);

        let mut components = VPath::new("/tmp/foo.txt///").reverse_components();
        assert_eq!(components.as_path(), "/tmp/foo.txt///");
        assert_eq!(components.next(), Some(Component::Segment("foo.txt")));
        assert_eq!(components.as_path(), "/tmp");
        assert_eq!(components.next(), Some(Component::Segment("tmp")));
        assert_eq!(components.as_path(), "/");
        assert_eq!(components.next(), Some(Component::Root));
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);

        let mut components = VPath::new("//tmp////foo.txt///").reverse_components();
        assert_eq!(components.as_path(), "//tmp////foo.txt///");
        assert_eq!(components.next(), Some(Component::Segment("foo.txt")));
        assert_eq!(components.as_path(), "//tmp");
        assert_eq!(components.next(), Some(Component::Segment("tmp")));
        assert_eq!(components.as_path(), "/");
        assert_eq!(components.next(), Some(Component::Root));
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);

        let mut components = VPath::new("tmp/foo.txt").reverse_components();
        assert_eq!(components.as_path(), "tmp/foo.txt");
        assert_eq!(components.next(), Some(Component::Segment("foo.txt")));
        assert_eq!(components.as_path(), "tmp");
        assert_eq!(components.next(), Some(Component::Segment("tmp")));
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);

        let mut components = VPath::new("tmp").reverse_components();
        assert_eq!(components.as_path(), "tmp");
        assert_eq!(components.next(), Some(Component::Segment("tmp")));
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);

        let mut components = VPath::new("").reverse_components();
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);

        let mut components = VPath::new("/").reverse_components();
        assert_eq!(components.as_path(), "/");
        assert_eq!(components.next(), Some(Component::Root));
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);
    }

    #[test]
    pub fn components_test() {
        use super::*;

        let mut components = VPath::new("/tmp/foo.txt").components();
        assert_eq!(components.as_path(), "/tmp/foo.txt");
        assert_eq!(components.next(), Some(Component::Root));
        assert_eq!(components.as_path(), "tmp/foo.txt");
        assert_eq!(components.next(), Some(Component::Segment("tmp")));
        assert_eq!(components.as_path(), "foo.txt");
        assert_eq!(components.next(), Some(Component::Segment("foo.txt")));
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);

        let mut components = VPath::new("/tmp/foo.txt///").components();
        assert_eq!(components.as_path(), "/tmp/foo.txt///");
        assert_eq!(components.next(), Some(Component::Root));
        assert_eq!(components.as_path(), "tmp/foo.txt///");
        assert_eq!(components.next(), Some(Component::Segment("tmp")));
        assert_eq!(components.as_path(), "foo.txt///");
        assert_eq!(components.next(), Some(Component::Segment("foo.txt")));
        assert_eq!(components.as_path(), "");

        let mut components = VPath::new("//tmp////foo.txt///").components();
        assert_eq!(components.as_path(), "//tmp////foo.txt///");
        assert_eq!(components.next(), Some(Component::Root));
        assert_eq!(components.as_path(), "tmp////foo.txt///");
        assert_eq!(components.next(), Some(Component::Segment("tmp")));
        assert_eq!(components.as_path(), "foo.txt///");
        assert_eq!(components.next(), Some(Component::Segment("foo.txt")));
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);

        let mut components = VPath::new("tmp/foo.txt").components();
        assert_eq!(components.as_path(), "tmp/foo.txt");
        assert_eq!(components.next(), Some(Component::Segment("tmp")));
        assert_eq!(components.as_path(), "foo.txt");
        assert_eq!(components.next(), Some(Component::Segment("foo.txt")));
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);

        let mut components = VPath::new("tmp").components();
        assert_eq!(components.as_path(), "tmp");
        assert_eq!(components.next(), Some(Component::Segment("tmp")));
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);

        let mut components = VPath::new("").components();
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);

        let mut components = VPath::new("/").components();
        assert_eq!(components.as_path(), "/");
        assert_eq!(components.next(), Some(Component::Root));
        assert_eq!(components.as_path(), "");
        assert_eq!(components.next(), None);
    }

    #[test]
    pub fn trim_trailing_sep_test() {
        use super::*;

        assert_eq!(VPath::new("dir//").trim_trailing_sep().to_str(), "dir");
        assert_eq!(VPath::new("dir/").trim_trailing_sep().to_str(), "dir");
        assert_eq!(VPath::new("dir").trim_trailing_sep().to_str(), "dir");
        assert_eq!(VPath::new("/").trim_trailing_sep().to_str(), "/");
        assert_eq!(VPath::new("//").trim_trailing_sep().to_str(), "//");
    }

    #[test]
    pub fn parent_test() {
        use super::*;

        let path = VPath::new("/foo/bar");
        let parent = path.parent().unwrap();
        assert_eq!(parent, VPath::new("/foo"));

        let grand_parent = parent.parent().unwrap();
        assert_eq!(grand_parent, VPath::new("/"));
        assert_eq!(grand_parent.parent(), None);

        let relative_path = VPath::new("foo/bar");
        let parent = relative_path.parent();
        assert_eq!(parent, Some(VPath::new("foo")));
        let grand_parent = parent.and_then(VPath::parent);
        assert_eq!(grand_parent, Some(VPath::new("")));
        let great_grand_parent = grand_parent.and_then(VPath::parent);
        assert_eq!(great_grand_parent, None);
    }
}
