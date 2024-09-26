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

use clap::{Arg, ArgMatches, Command};

use crate::commands::ISubcommand;
use crate::project::AlephProject;

pub struct Uuid {}

impl ISubcommand for Uuid {
    fn name(&self) -> &'static str {
        "uuid"
    }

    fn description(&mut self) -> Command {
        let count = Arg::new("count")
            .index(1)
            .help("The number of UUIDs to generate")
            .default_value("1")
            .required(false);
        Command::new(self.name())
            .about("Generate a UUIDv7 and write it out to stdout")
            .arg(count)
    }

    fn exec(&mut self, _project: &AlephProject, mut matches: ArgMatches) -> anyhow::Result<()> {
        let count: String = matches.remove_one("count").expect("count is required");
        let count: usize = count.parse().expect("Unable to parse 'count' argument");

        for _ in 0..count {
            let id = uuid::Uuid::now_v7();
            println!("{}", id);
        }

        Ok(())
    }

    fn dont_log(&self) -> bool {
        true
    }
}
