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

///
/// Embedded bytes of CascadiaCode.ttf
///
pub fn cascadia_code() -> &'static [u8] {
    include_bytes!("../fonts/CascadiaCode.ttf")
}

///
/// Embedded bytes of emoji-icon-font.ttf
///
pub fn emoji_icon_font() -> &'static [u8] {
    include_bytes!("../fonts/emoji-icon-font.ttf")
}

///
/// Embedded bytes of JetBrainsMono-Regular.ttf
///
pub fn jetbrains_mono_regular() -> &'static [u8] {
    include_bytes!("../fonts/JetBrainsMono-Regular.ttf")
}

///
/// Embedded bytes of NotoEmoji-Regular.ttf
///
pub fn noto_emoji_regular() -> &'static [u8] {
    include_bytes!("../fonts/NotoEmoji-Regular.ttf")
}

///
/// Embedded bytes of NotoSans-Regular.ttf
///
pub fn noto_sans_regular() -> &'static [u8] {
    include_bytes!("../fonts/NotoSans-Regular.ttf")
}
