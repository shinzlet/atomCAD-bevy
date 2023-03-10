// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate relaunch;

use relaunch::Trampoline;

fn main() {
    const APP_NAME: &str = "atomCAD";

    // If we are running from the command line (e.g. as a result of `cargo
    // run`), relaunch as a dynamically created app bundle.  This is required
    // because many Cocoa APIs will not work unless the application is running
    // from a bundle.
    #[cfg(target_os = "macos")]
    let _ =
        match Trampoline::new(&APP_NAME, "io.atomcad.atomCAD").bundle(relaunch::InstallDir::Temp) {
            Err(e) => {
                // We can't read/write to the filesystem?  This is a fatal error.
                println!("IO error! {}", e);
                std::process::exit(1);
            }
            Ok(app) => app,
        };
}

// End of File
