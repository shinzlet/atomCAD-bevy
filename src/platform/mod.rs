// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

use crate::platform_impl;
use std::io::Error as IOError;

pub struct Trampoline;
pub use platform_impl::relaunch::Application;

impl Trampoline {
    pub fn new(name: &str, ident: &str, version: &str) -> Result<Application, IOError> {
        platform_impl::relaunch::Trampoline::new(name, ident, version)
    }
}

// End of File
