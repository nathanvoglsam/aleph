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

extern crate aleph_quickjs as qjs;

use std::env::{current_dir, current_exe};
use std::io;
use std::str::FromStr;

use aleph_nstr::{nstr, NStr};
use aleph_target::build::{target_architecture, target_build_type, target_platform};
use camino::{Utf8Path, Utf8PathBuf};
use qjs::ToRefValue;
use thiserror::Error;

pub struct ConfigRunner {
    /// The QJS runtime that all the script context we run will be spawned inside of
    runtime: qjs::Runtime,

    /// The directory we will load config scripts from
    config_dir: Utf8PathBuf,

    /// A reference to the quickjs object that stores the config
    config_object: qjs::Object,
}

impl ConfigRunner {
    pub fn new() -> io::Result<Self> {
        let runtime = qjs::Runtime::new().unwrap();
        let context = runtime.new_context().unwrap();
        let config_object = context.new_object();
        let config_dir = find_folder_in_search_path("configs")?;

        let out = Self {
            runtime,
            config_dir,
            config_object,
        };
        Ok(out)
    }

    pub fn run_all_configs(&mut self) -> Result<(), RunConfigError> {
        for item in self.config_dir.read_dir_utf8()? {
            let item = item?;

            // Skip '@' configs as those are overrides not config scripts
            if item.file_name().starts_with('@') {
                continue;
            }

            // Skip non js files
            if !item.file_name().ends_with(".js") {
                continue;
            }

            let file_type = item.file_type()?;
            if file_type.is_file() || file_type.is_symlink() {
                let name = item.path().file_stem().unwrap();
                self.run_config_by_name(name)?;
            }
        }

        Ok(())
    }

    pub fn run_config_by_name(&mut self, name: &str) -> Result<(), RunConfigError> {
        let script = self.load_config_script(name)?;
        let script_nstr = NStr::from_str(&script).unwrap();

        let filename = format!("{name}.js\0");
        let filename = NStr::from_str(&filename).unwrap();

        // We create a new context for every script so they don't polute eachother's global state.
        //
        // Objects are free to be shared between contexts within the same runtime so we hold on to
        // the config object though
        let context = self.runtime.new_context().unwrap();

        // Provide the 'Configs' object which is where the scripts are expected to write their
        // config into.

        let global = context.get_global_object();
        if 0 > global.set_property_str("Configs", &self.config_object) {
            return Err(js_exception_to_err(&context));
        }

        Self::setup_global_environment(&context)?;

        // Run the setup script to provide the default functions
        log::trace!("Running aleph-config.js");
        let result = context.eval(
            SETUP_SCRIPT,
            nstr!("aleph-config.js"),
            qjs::raw::JSEvalOptions::STRICT,
        );
        let _ = check_exception(&context, result)?;

        // Load the script module itself. This won't run the config script, just load all the code
        // into the context. We have to fetch the entry point and execute the entry point
        // separately...
        log::trace!("Running {filename}");
        let result = context.eval(script_nstr, filename, qjs::raw::JSEvalOptions::STRICT);
        let _ = check_exception(&context, result)?;

        Ok(())
    }

    pub fn run_override_script(&mut self) -> Result<(), RunConfigError> {
        let script = self.load_config_script("@game")?;
        let script_nstr = NStr::from_str(&script).unwrap();

        let filename = nstr!("@game.js");

        // We create a new context for every script so they don't polute eachother's global state.
        //
        // Objects are free to be shared between contexts within the same runtime so we hold on to
        // the config object though
        let context = self.runtime.new_context().unwrap();

        // Provide the 'Configs' object which is where the scripts are expected to write their
        // config into.
        let global = context.get_global_object();
        if 0 > global.set_property_str("Configs", &self.config_object) {
            return Err(js_exception_to_err(&context));
        }

        Self::setup_global_environment(&context)?;

        // Load the script module itself. This won't run the config script, just load all the code
        // into the context. We have to fetch the entry point and execute the entry point
        // separately...
        let result = context.eval(script_nstr, filename, qjs::raw::JSEvalOptions::STRICT);
        let _ = check_exception(&context, result)?;

        Ok(())
    }

    pub fn finalize(self) -> serde_json::Map<String, serde_json::Value> {
        let json = self.config_object.to_json().unwrap();
        let mut json = match json {
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_)
            | serde_json::Value::Array(_) => panic!("Unexpected serde_json::Value type"),
            serde_json::Value::Object(v) => v,
        };
        Self::command_line_overrides(&mut json);

        json
    }
}

impl ConfigRunner {
    fn command_line_overrides(json: &mut serde_json::Map<String, serde_json::Value>) {
        use serde_json::{Map, Value};

        let cfg_arg = clap::Arg::new("cfg")
            .long("cfg")
            .short('c')
            .num_args(2)
            .value_names(["config", "value"])
            .action(clap::ArgAction::Append);
        let command = clap::Command::new("").arg(cfg_arg);
        let matches = command.get_matches();

        if let Some(cfgs) = matches.get_occurrences("cfg") {
            let cfgs: Vec<Vec<&String>> = cfgs.map(Iterator::collect).collect();
            for cfg in cfgs {
                let (cfg, val) = (cfg[0], cfg[1]);

                // Sanitize for array syntax in case someone tries to get clever
                if cfg.contains('[') || cfg.contains(']') {
                    log::error!("Config name invalid '{cfg}'. Array syntax unsupported");
                    panic!("Config name invalid '{cfg}'. Array syntax unsupported");
                }

                // Parse the value to a serde_json::Value. We try and parse it directly first so we
                // can get a typed value (56 = number, true = boolean, etc). Quote handling on the
                // command line is a bit funny so we have to try this two staged approach as the
                // user will almost certainly provide unquoted strings.
                //
                // Anything that fails to parse directly gets coerced to a string.
                let value = serde_json::Value::from_str(val)
                    .unwrap_or_else(|_| serde_json::Value::String(val.clone()));

                // Split the config name path into the invividual segmants
                let segments: Vec<&str> = cfg.split('.').collect();
                assert!(!segments.is_empty());

                // Split the segments at the very tail as they need to be handled separately.
                //
                // Given 'seg1.seg2.seg3' this should yield path = [seg1, seg2] tail = [seg3]. The
                // base segments can only refer to objects (tables) as they're simply a set of
                // directions to find a value identified by 'tail'.
                let (path, tail) = segments.split_at(segments.len() - 1);

                // Follow our base path to find the object we want. This will create new objects in
                // the 'json' if they are missing.
                let mut trace = String::new();
                let mut current = &mut *json;
                for seg in path.iter().copied() {
                    trace.push_str(seg);

                    fn trace_err(cfg: &str, trace: &str, got: &str) -> ! {
                        log::error!(
                            "Bad config arg '{cfg}': Expected 'object' at '{trace}', got '{got}'"
                        );
                        panic!(
                            "Bad config arg '{cfg}': Expected 'object' at '{trace}', got '{got}'"
                        );
                    }

                    let prop = current
                        .entry(seg)
                        .or_insert_with(|| Value::Object(Map::new()));
                    match prop {
                        Value::Null => trace_err(cfg, &trace, "null"),
                        Value::Bool(_) => trace_err(cfg, &trace, "boolean"),
                        Value::Number(_) => trace_err(cfg, &trace, "number"),
                        Value::String(_) => trace_err(cfg, &trace, "string"),
                        Value::Array(_) => trace_err(cfg, &trace, "array"),
                        Value::Object(v) => current = v,
                    }

                    // Push the dot after so it doesn't get included in any error messages above
                    trace.push('.');
                }

                // Once we've handled the base section of the path then current should be pointing
                // at the object we want to assign the 'tail' field in. And that's all we have to
                // do. Assign/replace the key with the value we parsed earlier.
                current.insert(tail[0].to_string(), value);
            }
        }
    }

    fn setup_global_environment<'a>(context: &'a qjs::Context) -> Result<(), RunConfigError> {
        let global = context.get_global_object();

        let p_string = context.new_string(target_platform().name());
        let a_string = context.new_string(target_architecture().name());
        let b_string = context.new_string(target_build_type().name());

        if 0 > global.set_property_str("THIS_PLATFORM", &p_string) {
            return Err(js_exception_to_err(&context));
        }
        if 0 > global.set_property_str("THIS_ARCHITECTURE", &a_string) {
            return Err(js_exception_to_err(&context));
        }
        if 0 > global.set_property_str("THIS_BUILD_TYPE", &b_string) {
            return Err(js_exception_to_err(&context));
        }

        let p = context.new_object().to_ref_value();
        let a = context.new_object().to_ref_value();
        let b = context.new_object().to_ref_value();
        let e = context.new_object().to_ref_value();

        if 0 > global.set_property_str("Platform", &p) {
            return Err(js_exception_to_err(&context));
        }
        if 0 > global.set_property_str("Architecture", &a) {
            return Err(js_exception_to_err(&context));
        }
        if 0 > global.set_property_str("BuildType", &b) {
            return Err(js_exception_to_err(&context));
        }
        if 0 > global.set_property_str("Environment", &e) {
            return Err(js_exception_to_err(&context));
        }

        Ok(())
    }
}

impl ConfigRunner {
    /// Internal function for loading the config script under the given name from the script folder.
    fn load_config_script(&self, name: &str) -> Result<String, RunConfigError> {
        let config = self.config_dir.join(name).with_extension("js");

        // Check if the config file exists
        if !config.is_file() {
            return Err(RunConfigError::NoConfig);
        }

        let mut string = std::fs::read_to_string(config)?;
        string.push('\0');
        Ok(string)
    }
}

#[derive(Error, Debug)]
pub enum RunConfigError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("A JS error occured: {0}")]
    Js(String),

    #[error("No config with the given name was found")]
    NoConfig,
}

///
/// Attempts to find a folder by the given name in the standard aleph search path for 'data'.
///
/// Will search the following locations in the given order:
///
/// - `${CWD}/{folder}`
/// - `${EXE_DIR}/{folder}`
/// - `${CWD}/.aleph/{folder}`
///
/// # Info
///
/// This function will search in the 'cwd' and adjacent to the game executable first before guessing
/// we're running in a development environment and trying the '.aleph/data' folder directory. This
/// does bake our project layout in as an expectation but it's good enough.
///
fn find_folder_in_search_path(folder: &str) -> io::Result<Utf8PathBuf> {
    let cwd = current_dir()?;
    let cwd = Utf8Path::from_path(cwd.as_path()).unwrap();
    let cwd_config = cwd.join(folder);

    if cwd_config.is_dir() {
        return Ok(cwd_config);
    }

    let exe_dir = current_exe()?;
    let exe_dir = exe_dir.parent().unwrap();
    let exe_dir = Utf8Path::from_path(exe_dir).unwrap();
    let exe_dir_config = exe_dir.join(folder);

    if exe_dir_config.is_dir() {
        return Ok(exe_dir_config);
    }

    let dot_aleph_dir_config = cwd.join(".aleph").join(folder);
    if dot_aleph_dir_config.is_dir() {
        return Ok(dot_aleph_dir_config);
    }

    return Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Failed to find a '{folder}' directory in our search paths."),
    ));
}

fn check_exception(
    context: &qjs::Context,
    v: qjs::RefValue,
) -> Result<qjs::RefValue, RunConfigError> {
    if v.is_exception() {
        Err(js_exception_to_err(context))
    } else {
        Ok(v)
    }
}

fn js_exception_to_err(context: &qjs::Context) -> RunConfigError {
    let exception = context.get_exception();
    assert!(!exception.is_undefined());
    let message = exception
        .to_c_str()
        .expect("Failed to get exception message");
    RunConfigError::Js(message.to_string())
}

const SETUP_SCRIPT: &NStr = nstr!(include_str!("../config/config.js"));
