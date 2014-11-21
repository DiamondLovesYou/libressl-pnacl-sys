// Copyright (c) 2014 Richard Diamond & contributors.
//
// This file is part of the Rust PPApi project.
//
// Rust PPApi is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rust PPApi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Rust PPApi. If not, see <http://www.gnu.org/licenses/>.

extern crate "pnacl-build-helper" as helper;

use std::os::getcwd;

pub fn main() {
    let libs = [(Path::new("ssl/.libs/"), "ssl:static".to_string()),
                (Path::new("crypto/.libs/"), "crypto:static".to_string())];
    let mut cfg = helper::ConfigureMake::new
        (&["--disable-shared".to_string(),
           "--without-pic".to_string(),
           "CFLAGS=-DNO_SYSLOG".to_string()],
         &libs);

    cfg.make_only_dir(Path::new("ssl"))
        .make_only_dir(Path::new("crypto"));

    cfg.configure(Some(getcwd().unwrap().join("libressl")));
    cfg.make();
}
