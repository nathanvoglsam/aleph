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

use std::io::Cursor;

use zip::ZipArchive;

pub const ANDROID_PROJECT: &[u8] = include_bytes!("../templates/android.zip");
pub const UWP_PROJECT: &[u8] = include_bytes!("../templates/uwp.zip");

pub fn android_project_bundle() -> ZipArchive<Cursor<&'static [u8]>> {
    let reader = Cursor::new(ANDROID_PROJECT);
    ZipArchive::new(reader)
        .expect("Failed to open internal zip archive for bundled project template")
}

pub fn uwp_project_bundle() -> ZipArchive<Cursor<&'static [u8]>> {
    let reader = Cursor::new(UWP_PROJECT);
    ZipArchive::new(reader)
        .expect("Failed to open internal zip archive for bundled project template")
}

pub const ANDROID_ACTIVITY_SOURCE_TEMPLATE: &str = r#"
package {{ANDROID_GAME_APPLICATION_ID}};

import org.libsdl.app.SDLActivity;

public class AlephActivity extends SDLActivity {
    @Override
    protected String[] getLibraries() {
        return new String[] {
                "SDL2",
                // "SDL2_image",
                // "SDL2_mixer",
                // "SDL2_net",
                // "SDL2_ttf",
                "{{ANDROID_GAME_LIBRARY}}"
        };
    }
}
"#;

pub const LOCAL_PROPERTIES_TEMPLATE: &str = r#"
## This file must *NOT* be checked into Version Control Systems,
# as it contains information specific to your local configuration.
#
# Location of the SDK. This is only used by Gradle.
# For customization when using a Version Control System, please read the
# header note.
#Sun Jun 18 15:19:45 AEST 2023
## Set these to your android SDK and NDK directories, e.g.
## sdk.dir=C\:\\Users\\Nathan\\AppData\\Local\\Android\\Sdk
## ndk.dir=C\:\\Users\\Nathan\\Programs\\Loose\\android-ndk-r25c
"#;

pub const SHADER_NINJA_RULES: &str = include_str!("../templates/shader_rules.ninja");
