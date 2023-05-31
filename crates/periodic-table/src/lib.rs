// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use bevy::prelude::*;
use static_assertions::const_assert_eq;
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)] // Oganesson == 118
pub enum Element {
    Hydrogen = 1,
    Helium,
    Lithium,
    Beryllium,
    Boron,
    Carbon,
    Nitrogen,
    Oxygen,
    Fluorine,
    Neon,
    Sodium,
    Magnesium,
    Aluminium,
    Silicon,
    Phosphorus,
    Sulfur,
    Chlorine,
    Argon,
    Potassium,
    Calcium,
    Scandium,
    Titanium,
    Vanadium,
    Chromium,
    Manganese,
    Iron,
    Cobalt,
    Nickel,
    Copper,
    Zinc,
    Gallium,
    Germanium,
    Arsenic,
    Selenium,
    Bromine,
    Krypton,
    Rubidium,
    Strontium,
    Yttrium,
    Zirconium,
    Niobium,
    Molybdenum,
    Technetium,
    Ruthenium,
    Rhodium,
    Palladium,
    Silver,
    Cadmium,
    Indium,
    Tin,
    Antimony,
    Tellurium,
    Iodine,
    Xenon,
    Cesium,
    Barium,
    Lanthanum,
    Cerium,
    Praseodymium,
    Neodymium,
    Promethium,
    Samarium,
    Europium,
    Gadolinium,
    Terbium,
    Dysprosium,
    Holmium,
    Erbium,
    Thulium,
    Ytterbium,
    Lutetium,
    Hafnium,
    Tantalum,
    Tungsten,
    Rhenium,
    Osmium,
    Iridium,
    Platinum,
    Gold,
    Mercury,
    Thallium,
    Lead,
    Bismuth,
    Polonium,
    Astatine,
    Radon,
    Francium,
    Radium,
    Actinium,
    Thorium,
    Protactinium,
    Uranium,
    Neptunium,
    Plutonium,
    Americium,
    Curium,
    Berkelium,
    Californium,
    Einsteinium,
    Fermium,
    Mendelevium,
    Nobelium,
    Lawrencium,
    Rutherfordium,
    Dubnium,
    Seaborgium,
    Bohrium,
    Hassium,
    Meitnerium,
    Darmstadtium,
    Roentgenium,
    Copernicium,
    Nihonium,
    Flerovium,
    Moscovium,
    Livermorium,
    Tennessine,
    Oganesson,
}
const_assert_eq!(Element::Oganesson as usize, 118);

impl Element {
    pub const MIN: Self = Element::Hydrogen; // 1
    pub const MAX: Self = Element::Oganesson; // 118

    pub fn from_atomic_number(n: u8) -> Option<Self> {
        if Self::MIN as u8 <= n && n <= Self::MAX as u8 {
            Some(unsafe { mem::transmute(n) })
        } else {
            None
        }
    }
}

pub struct PeriodicTable {
    pub element_reprs: Vec<ElementRepr>,
}

impl PeriodicTable {
    pub fn new() -> Self {
        let mut element_reprs = vec![
            ElementRepr {
                color: Vec3::new(0.0, 0.0, 0.0), // Black
                radius: 1.0,
            };
            118
        ];
        element_reprs[Element::Hydrogen as usize - 1] = ElementRepr {
            color: Vec3::new(0.8510, 0.8510, 0.8510), // white
            radius: 1.2,
        };
        element_reprs[Element::Carbon as usize - 1] = ElementRepr {
            color: Vec3::new(0.30196, 0.2902, 0.3098), // dark grey
            radius: 1.7,
        };
        element_reprs[Element::Oxygen as usize - 1] = ElementRepr {
            color: Vec3::new(0.7490, 0.2118, 0.3176), // red
            radius: 1.52,
        };
        element_reprs[Element::Silicon as usize - 1] = ElementRepr {
            // color: Vec3::new(0.7294, 0.5804, 0.1686), // yellow
            color: Vec3::new(0.5234, 0.5234, 0.5234), // light grey
            radius: 2.1,
        };
        element_reprs[Element::Phosphorus as usize - 1] = ElementRepr {
            color: Vec3::new(0.7019, 0.4314, 0.1451), // orange
            radius: 1.8,
        };
        element_reprs[Element::Nitrogen as usize - 1] = ElementRepr {
            color: Vec3::new(0.2078, 0.4549, 0.6118), // blue
            radius: 1.55,
        };
        element_reprs[Element::Sulfur as usize - 1] = ElementRepr {
            color: Vec3::new(0.7294, 0.5804, 0.1686), // yellow
            radius: 1.8,
        };

        Self { element_reprs }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ElementRepr {
    color: Vec3, // RGB color space
    radius: f32, // in angstroms
}
const_assert_eq!(mem::size_of::<ElementRepr>(), 16);

// End of File
