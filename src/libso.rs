/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 */

#![doc(
    html_logo_url = "https://iot.bzh/images/defaults/company/512-479-max-transp.png",
    html_favicon_url = "https://iot.bzh/images/defaults/favicon.ico"
)]

#[cfg(test)]
#[path = "../test/test-widgets.rs"]
mod test;

#[path = "../capi/capi-mod.rs"]
mod capi;

#[path = "assets.rs"]
mod assets;

#[path = "generic-enums.rs"]
mod generic;

#[path = "shared-methods.rs"]
mod shared;

#[path = "core-widgets.rs"]
mod core;

#[path = "extra-widgets.rs"]
mod extra;

pub mod prelude {
    pub(crate) use crate::capi::*;
    pub use crate::capi::get_time;
    pub use crate::assets::*;
    pub use crate::shared::*;
    pub use crate::generic::*;
    pub use crate::core::*;
    pub use crate::extra::*;
}