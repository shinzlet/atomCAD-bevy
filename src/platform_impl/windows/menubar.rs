// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::menubar::Menu;
use bevy::winit::WinitWindows;

// Currently does nothing, and is present merely to ensure we compile on
// Windows.  Should use the win32 APIs to setup the menubar for the main
// window(s).
pub fn attach_menu(_windows: &WinitWindows, _menu: &Menu) {}

// End of File
