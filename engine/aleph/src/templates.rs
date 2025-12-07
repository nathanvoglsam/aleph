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

pub const SHADER_NINJA_RULES: &str = include_str!("../templates/shader_rules.ninja");

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypescriptReference {
    pub path: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypescriptCompilerOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lib: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_strict: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_comments: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_emit: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_implicit_any: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_implicit_returns: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_implicit_override: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_error_truncation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_file: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypescriptConfig {
    pub compiler_options: TypescriptCompilerOptions,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<TypescriptReference>>,
}

pub fn root_tsconfig_template() -> TypescriptConfig {
    TypescriptConfig {
        compiler_options: TypescriptCompilerOptions {
            composite: None,
            target: None,
            lib: None,
            strict: None,
            always_strict: None,
            remove_comments: None,
            no_emit: Some(true),
            no_implicit_any: None,
            no_implicit_returns: None,
            no_implicit_override: None,
            no_error_truncation: None,
            out_file: None,
        },
        include: Some(Vec::new()),
        exclude: Some(vec!["**/*".to_string()]),
        references: None,
    }
}

pub fn configs_tsconfig_template() -> TypescriptConfig {
    TypescriptConfig {
        compiler_options: TypescriptCompilerOptions {
            composite: Some(true),
            target: Some("ES2020".to_string()),
            lib: Some(vec!["ES2020".to_string()]),
            strict: Some(true),
            always_strict: Some(true),
            remove_comments: Some(true),
            no_emit: None,
            no_implicit_any: Some(true),
            no_implicit_returns: Some(true),
            no_implicit_override: Some(true),
            no_error_truncation: Some(true),
            out_file: None,
        },
        include: None,
        exclude: None,
        references: None,
    }
}
