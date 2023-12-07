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
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::prelude::*;


macro_rules! impl_static_palette {
    ($palette:ident) => {
        pub fn $palette() -> Self {
          LvglColor::palette(LvglPalette::$palette)
        }
    }
}

pub struct LvglColor {
    pub(crate) handle: cglue::lv_color_t,
}
#[allow(non_snake_case)]
impl LvglColor {
    pub fn rvb(red: u8, green: u8, blue: u8) -> Self {
        let handle = unsafe { cglue::lv_color_mk(red, green, blue) };
        LvglColor { handle }
    }

    fn palette(palette: u32) -> Self {
        let handle = unsafe { cglue::lv_palette_main(palette) };
        LvglColor { handle }
    }

    impl_static_palette!(RED);
    impl_static_palette!(PINK);
    impl_static_palette!(PURPLE);
    impl_static_palette!(DEEP_PURPLE);
    impl_static_palette!(INDIGO);
    impl_static_palette!(BLUE);
    impl_static_palette!(LIGHT_BLUE);
    impl_static_palette!(CYAN);
    impl_static_palette!(TEAL);
    impl_static_palette!(GREEN);
    impl_static_palette!(LIGHT_GREEN);
    impl_static_palette!(LIME);
    impl_static_palette!(YELLOW);
    impl_static_palette!(AMBER);
    impl_static_palette!(ORANGE);
    impl_static_palette!(DEEP_ORANGE);
    impl_static_palette!(BROWN);
    impl_static_palette!(BLUE_GREY);
    impl_static_palette!(GREY);
    impl_static_palette!(LAST);
    impl_static_palette!(NONE);
}

pub struct LvglPixmap;
impl LvglPixmap {
    #![allow(dead_code)]
    pub const DUMMY: &'static [u8; 4] = b"\xEF\xA3\xBF\0";
    pub const BULLET: &'static [u8; 4] = b"\xE2\x80\xA2\0";
    pub const AUDIO: &'static [u8; 4] = b"\xEF\x80\x81\0";
    pub const VIDEO: &'static [u8; 4] = b"\xEF\x80\x88\0";
    pub const LIST: &'static [u8; 4] = b"\xEF\x80\x8B\0";
    pub const OK: &'static [u8; 4] = b"\xEF\x80\x8C\0";
    pub const CLOSE: &'static [u8; 4] = b"\xEF\x80\x8D\0";
    pub const POWER: &'static [u8; 4] = b"\xEF\x80\x91\0";
    pub const SETTINGS: &'static [u8; 4] = b"\xEF\x80\x93\0";
    pub const HOME: &'static [u8; 4] = b"\xEF\x80\x95\0";
    pub const DOWNLOAD: &'static [u8; 4] = b"\xEF\x80\x99\0";
    pub const DRIVE: &'static [u8; 4] = b"\xEF\x80\x9C\0";
    pub const REFRESH: &'static [u8; 4] = b"\xEF\x80\xA1\0";
    pub const MUTE: &'static [u8; 4] = b"\xEF\x80\xA6\0";
    pub const VOLUME_MID: &'static [u8; 4] = b"\xEF\x80\xA7\0";
    pub const VOLUME_MAX: &'static [u8; 4] = b"\xEF\x80\xA8\0";
    pub const IMAGE: &'static [u8; 4] = b"\xEF\x80\xBE\0";
    pub const TINT: &'static [u8; 4] = b"\xEF\x81\x83\0";
    pub const PREV: &'static [u8; 4] = b"\xEF\x81\x88\0";
    pub const PLAY: &'static [u8; 4] = b"\xEF\x81\x8B\0";
    pub const PAUSE: &'static [u8; 4] = b"\xEF\x81\x8C\0";
    pub const STOP: &'static [u8; 4] = b"\xEF\x81\x8D\0";
    pub const NEXT: &'static [u8; 4] = b"\xEF\x81\x91\0";
    pub const EJECT: &'static [u8; 4] = b"\xEF\x81\x92\0";
    pub const LEFT: &'static [u8; 4] = b"\xEF\x81\x93\0";
    pub const RIGHT: &'static [u8; 4] = b"\xEF\x81\x94\0";
    pub const PLUS: &'static [u8; 4] = b"\xEF\x81\xA7\0";
    pub const MINUS: &'static [u8; 4] = b"\xEF\x81\xA8\0";
    pub const EYE_OPEN: &'static [u8; 4] = b"\xEF\x81\xAE\0";
    pub const EYE_CLOSE: &'static [u8; 4] = b"\xEF\x81\xB0\0";
    pub const WARNING: &'static [u8; 4] = b"\xEF\x81\xB1\0";
    pub const SHUFFLE: &'static [u8; 4] = b"\xEF\x81\xB4\0";
    pub const UP: &'static [u8; 4] = b"\xEF\x81\xB7\0";
    pub const DOWN: &'static [u8; 4] = b"\xEF\x81\xB8\0";
    pub const LOOP: &'static [u8; 4] = b"\xEF\x81\xB9\0";
    pub const DIRECTORY: &'static [u8; 4] = b"\xEF\x81\xBB\0";
    pub const UPLOAD: &'static [u8; 4] = b"\xEF\x82\x93\0";
    pub const CALL: &'static [u8; 4] = b"\xEF\x82\x95\0";
    pub const CUT: &'static [u8; 4] = b"\xEF\x83\x84\0";
    pub const COPY: &'static [u8; 4] = b"\xEF\x83\x85\0";
    pub const SAVE: &'static [u8; 4] = b"\xEF\x83\x87\0";
    pub const BARS: &'static [u8; 4] = b"\xEF\x83\x89\0";
    pub const ENVELOPE: &'static [u8; 4] = b"\xEF\x83\xA0\0";
    pub const CHARGE: &'static [u8; 4] = b"\xEF\x83\xA7\0";
    pub const PASTE: &'static [u8; 4] = b"\xEF\x83\xAA\0";
    pub const BELL: &'static [u8; 4] = b"\xEF\x83\xB3\0";
    pub const KEYBOARD: &'static [u8; 4] = b"\xEF\x84\x9C\0";
    pub const GPS: &'static [u8; 4] = b"\xEF\x84\xA4\0";
    pub const FILE: &'static [u8; 4] = b"\xEF\x85\x9B\0";
    pub const WIFI: &'static [u8; 4] = b"\xEF\x87\xAB\0";
    pub const BATTERY_FULL: &'static [u8; 4] = b"\xEF\x89\x80\0";
    pub const BATTERY_3: &'static [u8; 4] = b"\xEF\x89\x81\0";
    pub const BATTERY_2: &'static [u8; 4] = b"\xEF\x89\x82\0";
    pub const BATTERY_1: &'static [u8; 4] = b"\xEF\x89\x83\0";
    pub const BATTERY_EMPTY: &'static [u8; 4] = b"\xEF\x89\x84\0";
    pub const USB: &'static [u8; 4] = b"\xEF\x8A\x87\0";
    pub const BLUETOOTH: &'static [u8; 4] = b"\xEF\x8A\x93\0";
    pub const TRASH: &'static [u8; 4] = b"\xEF\x8B\xAD\0";
    pub const EDIT: &'static [u8; 4] = b"\xEF\x8C\x84\0";
    pub const BACKSPACE: &'static [u8; 4] = b"\xEF\x95\x9A\0";
    pub const SD_CARD: &'static [u8; 4] = b"\xEF\x9F\x82\0";
    pub const NEW_LINE: &'static [u8; 4] = b"\xEF\xA2\xA2\0";
}

pub struct LvglPalette;
impl LvglPalette {
    pub const RED: cglue::lv_palette_t = 0;
    pub const PINK: cglue::lv_palette_t = 1;
    pub const PURPLE: cglue::lv_palette_t = 2;
    pub const DEEP_PURPLE: cglue::lv_palette_t = 3;
    pub const INDIGO: cglue::lv_palette_t = 4;
    pub const BLUE: cglue::lv_palette_t = 5;
    pub const LIGHT_BLUE: cglue::lv_palette_t = 6;
    pub const CYAN: cglue::lv_palette_t = 7;
    pub const TEAL: cglue::lv_palette_t = 8;
    pub const GREEN: cglue::lv_palette_t = 9;
    pub const LIGHT_GREEN: cglue::lv_palette_t = 10;
    pub const LIME: cglue::lv_palette_t = 11;
    pub const YELLOW: cglue::lv_palette_t = 12;
    pub const AMBER: cglue::lv_palette_t = 13;
    pub const ORANGE: cglue::lv_palette_t = 14;
    pub const DEEP_ORANGE: cglue::lv_palette_t = 15;
    pub const BROWN: cglue::lv_palette_t = 16;
    pub const BLUE_GREY: cglue::lv_palette_t = 17;
    pub const GREY: cglue::lv_palette_t = 18;
    pub const LAST: cglue::lv_palette_t = 19;
    pub const NONE: cglue::lv_palette_t = 255;
}

pub type LvglFont= cglue::lv_font_t;
macro_rules! impl_static_font {
    ($label:ident, $font:ident) => {
        pub fn $label() -> &'static LvglFont {
            unsafe { &cglue::$font }
        }
    }
}

pub struct LvglMkFont;
impl LvglMkFont {
    impl_static_font! (std_10, lv_font_montserrat_10);
    impl_static_font! (std_12, lv_font_montserrat_12);
    impl_static_font! (std_14, lv_font_montserrat_14);
    impl_static_font! (std_18, lv_font_montserrat_18);
    impl_static_font! (std_22, lv_font_montserrat_22);
    impl_static_font! (std_26, lv_font_montserrat_26);
    impl_static_font! (std_30, lv_font_montserrat_30);
    impl_static_font! (std_34, lv_font_montserrat_34);
    impl_static_font! (std_40, lv_font_montserrat_40);
    impl_static_font! (std_48, lv_font_montserrat_48);
}

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum LvglEvent {
    PRESSED,
    PRESSING,
    PRESS_LOST,
    SHORT_CLICKED,
    LONG_PRESSED,
    LONG_PRESSED_REPEAT,
    CLICKED,
    RELEASED,
    FOCUSED,
    DEFOCUSED,
    LEAVE,
    VALUE_CHANGED,
    UNKNOWN,
}

impl LvglEvent {
    pub(crate) fn from(code: u32) -> Self {
        match code {
            1 => Self::PRESSED,
            2 => Self::PRESSING,
            3 => Self::PRESS_LOST,
            4 => Self::SHORT_CLICKED,
            5 => Self::LONG_PRESSED,
            6 => Self::LONG_PRESSED_REPEAT,
            7 => Self::CLICKED,
            8 => Self::RELEASED,
            14 => Self::FOCUSED,
            15 => Self::DEFOCUSED,
            16 => Self::LEAVE,
            28 => Self::VALUE_CHANGED,

            _ => Self::UNKNOWN,
        }
    }
}
