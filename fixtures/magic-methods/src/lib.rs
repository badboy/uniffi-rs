/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#[derive(Debug, PartialEq, Hash)]
struct Magic {
    val: String,
}

impl Magic {
    fn new(val: String) -> Self {
        Self { val }
    }
}

impl std::fmt::Display for Magic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Magic({})", self.val)
    }
}

include!(concat!(env!("OUT_DIR"), "/magic_methods.uniffi.rs"));
