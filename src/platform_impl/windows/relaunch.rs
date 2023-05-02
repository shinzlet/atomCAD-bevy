// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

pub struct Trampoline;
pub struct Application;

impl Trampoline {
    pub fn new(_name: &str, _ident: &str, _version: &str) -> Result<Application, std::io::Error> {
        Ok(Application)
    }
}

// End of File
