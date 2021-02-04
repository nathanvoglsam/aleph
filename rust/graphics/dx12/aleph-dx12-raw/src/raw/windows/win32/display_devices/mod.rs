#[repr(C)]
#[allow(non_snake_case)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C)]
#[doc(hidden)]
pub struct RECT_abi(i32, i32, i32, i32);
impl RECT {}
unsafe impl ::windows::Abi for RECT {
    type Abi = RECT_abi;
}
impl ::std::default::Default for RECT {
    fn default() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
}
impl ::std::fmt::Debug for RECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RECT")
            .field("left", &format_args!("{:?}", self.left))
            .field("top", &format_args!("{:?}", self.top))
            .field("right", &format_args!("{:?}", self.right))
            .field("bottom", &format_args!("{:?}", self.bottom))
            .finish()
    }
}
impl ::std::clone::Clone for RECT {
    fn clone(&self) -> Self {
        Self {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }
}
impl ::std::cmp::PartialEq for RECT {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left
            && self.top == other.top
            && self.right == other.right
            && self.bottom == other.bottom
    }
}
impl ::std::cmp::Eq for RECT {}
impl ::std::marker::Copy for RECT {}
#[repr(C)]
#[allow(non_snake_case)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[doc(hidden)]
pub struct POINT_abi(i32, i32);
impl POINT {}
unsafe impl ::windows::Abi for POINT {
    type Abi = POINT_abi;
}
impl ::std::default::Default for POINT {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}
impl ::std::fmt::Debug for POINT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POINT")
            .field("x", &format_args!("{:?}", self.x))
            .field("y", &format_args!("{:?}", self.y))
            .finish()
    }
}
impl ::std::clone::Clone for POINT {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}
impl ::std::cmp::PartialEq for POINT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::std::cmp::Eq for POINT {}
impl ::std::marker::Copy for POINT {}
