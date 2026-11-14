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
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, bail};

pub const ANDROID_ACTIVITY_SOURCE_TEMPLATE: &str =
    include_str!("../../../templates/ActivityTemplate.java");

/// Represents the type of content a template file can have, either text or binary.
///
/// Text files are represented as strings, and are the only type of file that can have template
/// replacements applied.
enum Content {
    Text(&'static str),
    Binary(&'static [u8]),
}

/// Represents a template file, including its path, content, and whether it should be executable.
struct TemplateFile {
    /// Specifies the relative path of the file within the template.
    path: &'static str,

    /// Holds the actual content of the file, either as text or binary.
    content: Content,

    /// Indicates whether the file should be marked as executable.
    executable: bool,
}

macro_rules! text_file {
    ($path:literal) => {
        TemplateFile {
            path: $path,
            content: Content::Text(include_str!(concat!(
                "../../../templates/android-project/",
                $path
            ))),
            executable: false,
        }
    };
    ($path:literal, exe) => {
        TemplateFile {
            path: $path,
            content: Content::Text(include_str!(concat!(
                "../../../templates/android-project/",
                $path
            ))),
            executable: true,
        }
    };
}

macro_rules! binary_file {
    ($path:literal) => {
        TemplateFile {
            path: $path,
            content: Content::Binary(include_bytes!(concat!(
                "../../../templates/android-project/",
                $path
            ))),
            executable: false,
        }
    };
    ($path:literal, exe) => {
        TemplateFile {
            path: $path,
            content: Content::Binary(include_bytes!(concat!(
                "../../../templates/android-project/",
                $path
            ))),
            executable: true,
        }
    };
}

const FILES: &[TemplateFile] = &[
    text_file!("app/build.gradle"),
    text_file!("app/jni/CMakeLists.txt"),
    text_file!("app/jni/main.cpp"),
    text_file!("app/proguard-rules.pro"),
    text_file!("app/src/main/AndroidManifest.xml"),
    text_file!("app/src/main/java/org/libsdl/app/HIDDevice.java"),
    text_file!("app/src/main/java/org/libsdl/app/HIDDeviceBLESteamController.java"),
    text_file!("app/src/main/java/org/libsdl/app/HIDDeviceManager.java"),
    text_file!("app/src/main/java/org/libsdl/app/HIDDeviceUSB.java"),
    text_file!("app/src/main/java/org/libsdl/app/SDL.java"),
    text_file!("app/src/main/java/org/libsdl/app/SDLActivity.java"),
    text_file!("app/src/main/java/org/libsdl/app/SDLAudioManager.java"),
    text_file!("app/src/main/java/org/libsdl/app/SDLControllerManager.java"),
    text_file!("app/src/main/java/org/libsdl/app/SDLDummyEdit.java"),
    text_file!("app/src/main/java/org/libsdl/app/SDLInputConnection.java"),
    text_file!("app/src/main/java/org/libsdl/app/SDLSensorManager.java"),
    text_file!("app/src/main/java/org/libsdl/app/SDLSurface.java"),
    text_file!("app/src/main/res/values/colors.xml"),
    text_file!("app/src/main/res/values/strings.xml"),
    text_file!("app/src/main/res/values/styles.xml"),
    text_file!("build.gradle"),
    text_file!("gradle.properties"),
    binary_file!("gradle/wrapper/gradle-wrapper.jar"),
    text_file!("gradle/wrapper/gradle-wrapper.properties"),
    text_file!("gradlew", exe),
    text_file!("gradlew.bat"),
    text_file!("settings.gradle"),
];

pub fn render<'a>(
    template: &'a str,
    variables: &HashMap<String, String>,
) -> anyhow::Result<Cow<'a, str>> {
    // If the template does not contain any placeholders, return it as-is. Avoids copying the file
    // when we don't actually need to.
    if !template.contains("${{") {
        return Ok(Cow::Borrowed(template));
    }

    let mut rendered = String::with_capacity(template.len());
    let mut remaining = template;

    while let Some(start) = remaining.find("${{") {
        rendered.push_str(&remaining[..start]);
        remaining = &remaining[start + 3..];
        let Some(end) = remaining.find("}}") else {
            bail!("unterminated template placeholder");
        };
        let name = &remaining[..end];
        let value = variables
            .get(name)
            .with_context(|| format!("missing template variable {name:?}"))?;
        rendered.push_str(value);
        remaining = &remaining[end + 2..];
    }

    rendered.push_str(remaining);
    Ok(Cow::Owned(rendered))
}

fn render_content<'a>(
    content: &'a Content,
    variables: &HashMap<String, String>,
) -> anyhow::Result<Cow<'a, [u8]>> {
    match content {
        Content::Text(text) => Ok(match render(text, variables)? {
            Cow::Borrowed(text) => Cow::Borrowed(text.as_bytes()),
            Cow::Owned(text) => Cow::Owned(text.into_bytes()),
        }),
        Content::Binary(bytes) => Ok(Cow::Borrowed(bytes)),
    }
}

/// Recreates the embedded Android project tree in `destination`.
///
/// Variables use bare names, such as `ANDROID_GAME_NAME`, and values are inserted literally.
///
/// Missing variables are reported before any files are written. Existing template files are
/// overwritten, while unrelated files are left untouched. I/O errors can leave partial output.
pub fn write_android_project(
    destination: impl AsRef<Path>,
    variables: &HashMap<String, String>,
) -> anyhow::Result<()> {
    let files = FILES
        .iter()
        .map(|file| {
            let contents = render_content(&file.content, variables)
                .with_context(|| format!("rendering Android template {}", file.path))?;
            Ok((file, contents))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let destination = destination.as_ref();
    for (file, contents) in files {
        let path = destination.join(file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("creating directory {}", parent.display()))?;
        }
        fs::write(&path, contents).with_context(|| format!("writing {}", path.display()))?;
        #[cfg(unix)]
        if file.executable {
            use std::os::unix::fs::PermissionsExt;

            fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
                .with_context(|| format!("setting permissions on {}", path.display()))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::collections::{BTreeSet, HashMap};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::{Content, FILES, TemplateFile, render, render_content, write_android_project};

    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            static NEXT_ID: AtomicU64 = AtomicU64::new(0);
            let path = std::env::temp_dir().join(format!(
                "aleph-android-template-{}-{}",
                std::process::id(),
                NEXT_ID.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).expect("create isolated test directory");
            Self(path)
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).expect("remove isolated test directory");
        }
    }

    fn variables() -> HashMap<String, String> {
        [
            ("ANDROID_GAME_NAME", "MyGame"),
            ("ANDROID_GAME_PACKAGE", "org.example.game"),
            ("ALEPH_WORKSPACE_ROOT", "/workspace"),
            ("ANDROID_GAME_CRATE", "my-game"),
            ("ANDROID_GAME_TARGET", "my_game"),
        ]
        .into_iter()
        .map(|(name, value)| (name.to_owned(), value.to_owned()))
        .collect()
    }

    fn files_under(root: &Path) -> BTreeSet<PathBuf> {
        fn visit(root: &Path, directory: &Path, files: &mut BTreeSet<PathBuf>) {
            for entry in fs::read_dir(directory).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if entry.file_type().unwrap().is_dir() {
                    visit(root, &path, files);
                } else {
                    files.insert(path.strip_prefix(root).unwrap().to_owned());
                }
            }
        }

        let mut files = BTreeSet::new();
        visit(root, root, &mut files);
        files
    }

    #[test]
    fn binary_file_macro_supports_executable_flag() {
        let regular = binary_file!("gradle/wrapper/gradle-wrapper.jar");
        let executable = binary_file!("gradle/wrapper/gradle-wrapper.jar", exe);
        assert_eq!(regular.path, executable.path);
        assert!(!regular.executable);
        assert!(executable.executable);
        match (regular.content, executable.content) {
            (Content::Binary(regular), Content::Binary(executable)) => {
                assert_eq!(regular, executable);
            }
            _ => panic!("binary_file must embed binary content"),
        }
    }

    #[test]
    fn writes_complete_rendered_tree_and_preserves_unrelated_files() {
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/android-project");
        let output = TestDir::new();
        let destination = output.0.join("project");
        fs::create_dir_all(destination.join("app")).unwrap();
        fs::write(destination.join("app/build.gradle"), "old content").unwrap();
        fs::write(destination.join("unrelated"), "keep").unwrap();

        let variables = variables();
        write_android_project(&destination, &variables).unwrap();

        let source_files = files_under(&source);
        let embedded_files: BTreeSet<_> =
            FILES.iter().map(|file| PathBuf::from(file.path)).collect();
        assert_eq!(
            embedded_files.len(),
            FILES.len(),
            "duplicate embedded paths"
        );
        assert_eq!(embedded_files, source_files);
        let mut expected_files = source_files.clone();
        expected_files.insert(PathBuf::from("unrelated"));
        assert_eq!(files_under(&destination), expected_files);
        assert_eq!(fs::read(destination.join("unrelated")).unwrap(), b"keep");

        for path in source_files {
            let original = fs::read(source.join(&path)).unwrap();
            let expected = if path == Path::new("gradle/wrapper/gradle-wrapper.jar") {
                original
            } else {
                let mut text = String::from_utf8(original).unwrap();
                for (name, value) in &variables {
                    text = text.replace(&format!("${{{{{name}}}}}"), value);
                }
                text.into_bytes()
            };
            assert_eq!(
                fs::read(destination.join(&path)).unwrap(),
                expected,
                "{path:?}"
            );
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let mode = fs::metadata(destination.join("gradlew"))
                .unwrap()
                .permissions()
                .mode();
            assert_eq!(mode & 0o777, 0o755);
        }
    }

    #[test]
    fn renders_repeated_variables_and_inserts_values_literally() {
        let variables = HashMap::from([
            ("NAME".to_owned(), "${{OTHER}}".to_owned()),
            ("OTHER".to_owned(), "not expanded".to_owned()),
        ]);
        assert_eq!(
            render("before ${{NAME}} / ${{NAME}} after", &variables).unwrap(),
            "before ${{OTHER}} / ${{OTHER}} after"
        );
        assert!(render("${{NAME", &variables).is_err());
    }

    #[test]
    fn borrows_unchanged_content_and_owns_rendered_text() {
        let variables = HashMap::from([("NAME".to_owned(), "value".to_owned())]);
        let plain = Content::Text("unchanged");
        let binary = Content::Binary(b"\0${{NAME}}\xff");
        let templated = Content::Text("hello ${{NAME}}");

        assert!(matches!(
            render_content(&plain, &variables).unwrap(),
            Cow::Borrowed(b"unchanged")
        ));
        assert!(matches!(
            render_content(&binary, &variables).unwrap(),
            Cow::Borrowed(b"\0${{NAME}}\xff")
        ));
        assert!(matches!(
            render_content(&templated, &variables).unwrap(),
            Cow::Owned(bytes) if bytes == b"hello value"
        ));
    }

    #[test]
    fn missing_variable_does_not_modify_destination() {
        let output = TestDir::new();
        let destination = output.0.join("project");
        fs::create_dir(&destination).unwrap();
        fs::write(destination.join("build.gradle"), "original").unwrap();
        let error = write_android_project(&destination, &HashMap::new()).unwrap_err();
        assert!(format!("{error:#}").contains("missing template variable"));
        assert_eq!(
            fs::read(destination.join("build.gradle")).unwrap(),
            b"original"
        );
        assert_eq!(
            files_under(&destination),
            BTreeSet::from([PathBuf::from("build.gradle")])
        );
    }

    #[test]
    fn reports_io_errors() {
        let output = TestDir::new();
        let destination = output.0.join("not-a-directory");
        fs::write(&destination, "file").unwrap();
        let error = write_android_project(&destination, &variables()).unwrap_err();
        let message = format!("{error:#}");
        assert!(message.contains("creating directory"));
        assert!(message.contains("not-a-directory"));
    }
}
