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

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectSchema<'a> {
    /// General game project configuration
    pub game: GameSchema<'a>,

    /// Windows specific project configuration
    pub windows: Option<WindowsSchema<'a>>,

    /// UWP specific project configuration
    pub uwp: Option<UwpSchema<'a>>,

    /// Android specific project configuration
    pub android: Option<AndroidSchema<'a>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameSchema<'a> {
    /// The name of the game, as a pretty string. Example: "My Cool Game"
    pub name: Cow<'a, str>,

    /// The name of the crate as specified in the Cargo.toml. Example "my-cool-game"
    pub crate_name: Cow<'a, str>,

    /// The name of the game author. Example: "My Cool Studio"
    pub author: Cow<'a, str>,

    /// A list of supported platforms that the project is allowed to target. Allowed values: "uwp",
    /// "android", "windows", "macos", "linux"
    pub target_platforms: Vec<Cow<'a, str>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowsSchema<'a> {
    /// Specification of branding information for the windows executable. This includes things like
    /// the .exe icon to use.
    pub branding: Option<WindowsBrandingSchema<'a>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UwpSchema<'a> {
    /// Specification of branding information for the UWP executable. This includes things like
    /// the app icon to use, as well as other iconography for the app.
    pub branding: Option<UwpBrandingSchema<'a>>,

    /// A path, relative to the "aleph-project.toml", to the .pfx file used to sign the output app
    /// bundles.
    pub certificate: Cow<'a, str>,

    pub identity_name: Cow<'a, str>,
    pub identity_publisher: Cow<'a, str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AndroidSchema<'a> {
    /// Specification of branding information for the Android app. This includes information like
    /// the app icons to use.
    pub branding: Option<AndroidBrandingSchema<'a>>,

    /// The app's ID. Example: "com.mycoolstudio.mycoolgame.game"
    pub app_id: Cow<'a, str>,

    /// The app's version code. Monotonically increasing version count for app releases.
    pub version_id: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowsBrandingSchema<'a> {
    /// A path, relative to the "aleph-project.toml", to the .ico file to use as the application's
    /// icon.
    pub icon: Cow<'a, str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UwpBrandingSchema<'a> {
    pub lock_screen_logo: Cow<'a, str>,
    pub splash_screen: Cow<'a, str>,
    pub square_44_x_44_logo: Cow<'a, str>,
    pub square_150_x_150_logo: Cow<'a, str>,
    pub store_logo: Cow<'a, str>,
    pub wide_310_x_150_logo: Cow<'a, str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AndroidBrandingSchema<'a> {
    pub icon_mdpi: Cow<'a, str>,
    pub icon_hdpi: Cow<'a, str>,
    pub icon_xhdpi: Cow<'a, str>,
    pub icon_xxhdpi: Cow<'a, str>,
    pub icon_xxxhdpi: Cow<'a, str>,
}

#[cfg(test)]
mod tests {
    use crate::project_schema::ProjectSchema;

    #[test]
    pub fn parse_schema() {
        let text = std::fs::read_to_string("./test_data/aleph-project.toml").unwrap();
        let _project: ProjectSchema = toml::from_str(&text).unwrap();
    }
}
