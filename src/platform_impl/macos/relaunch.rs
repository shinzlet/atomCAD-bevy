// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate relaunch;

use std::io::Error as IOError;

pub struct Trampoline;
pub use relaunch::Application;

impl Trampoline {
    pub fn new(name: &str, ident: &str, version: &str) -> Result<Application, IOError> {
        relaunch::Trampoline::new(name, ident)
            .version(version)
            .bundle(relaunch::InstallDir::Temp)
    }
}

// End of File
