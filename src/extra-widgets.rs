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
use crate::impl_widget_trait;
use std::any::Any;
use std::cell::Cell;
use std::mem;
use std::os::raw;
use crate::prelude::*;

pub struct LvglMeter {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    scale: *mut cglue::lv_meter_scale_t,
    needle: *mut cglue::lv_meter_indicator_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglMeter, Meter);
impl LvglMeter {
    pub fn new(
        parent: &LvglWidget,
        uid: &'static str,
        needle_width: u16,
        needle_ratio: i16,
        needle_color: LvglColor,
        x_ofs: i16,
        y_ofs: i16,
    ) -> &'static Self {
        unsafe {
            let handle = cglue::lv_meter_create(parent.get_handle());
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            // add scale
            let scale = cglue::lv_meter_add_scale(handle);

            // add needel
            let needle = cglue::lv_meter_add_needle_line(
                handle,
                scale,
                needle_width,
                needle_color.handle,
                needle_ratio,
            );

            let widget = LvglMeter {
                uid,
                info: Cell::new(""),
                handle,
                style,
                scale,
                needle,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_tic(
        &self,
        line_width: u16,
        label_gap: i16, // gap text to tick
        tick_count: u16,
        tick_length: u16,
        nth_major: u16, // number of tick to major
        minor_color: LvglColor,
        major_color: LvglColor,
    ) -> &Self {
        unsafe {
            cglue::lv_meter_set_scale_ticks(
                self.handle,
                self.scale,
                tick_count,
                line_width,
                tick_length,
                minor_color.handle,
            );
            cglue::lv_meter_set_scale_major_ticks(
                self.handle,
                self.scale,
                nth_major,
                (line_width as f32 * 1.5) as u16,
                (tick_length as f32 * 1.5) as u16,
                major_color.handle,
                label_gap,
            );
        }
        self
    }

    pub fn set_zone(&self, start: i32, end: i32, width: u16, color: LvglColor) -> &Self {
        unsafe {
            let indic = cglue::lv_meter_add_arc(self.handle, self.scale, width, color.handle, 0);
            cglue::lv_meter_set_indicator_start_value(self.handle, indic, start);
            cglue::lv_meter_set_indicator_end_value(self.handle, indic, end);
            let indic = cglue::lv_meter_add_scale_lines(
                self.handle,
                self.scale,
                color.handle,
                color.handle,
                false,
                0,
            );
            cglue::lv_meter_set_indicator_start_value(self.handle, indic, start);
            cglue::lv_meter_set_indicator_end_value(self.handle, indic, end);
        }
        self
    }

    pub fn set_value(&self, value: i32) -> &Self {
        unsafe {
            cglue::lv_meter_set_indicator_value(self.handle, self.needle, value);
        }
        self
    }
}

pub struct LvglQrcode {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
        style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglQrcode, Qrcode);
impl LvglQrcode {
    pub fn new(
        parent: &LvglWidget,
        uid: &'static str,
        dark_color: LvglColor,
        light_color: LvglColor,
        size: i16,
        x_ofs: i16,
        y_ofs: i16,
    ) -> &'static Self {
        unsafe {
            let handle = cglue::lv_qrcode_create(
                parent.get_handle(),
                size,
                dark_color.handle,
                light_color.handle,
            );
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            // create widget object and set text text
            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            let widget = LvglQrcode {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_value(&self, data: &str) -> &Self {
        unsafe {
            cglue::lv_qrcode_update(
                self.handle,
                data.as_bytes().as_ptr() as *const raw::c_void,
                data.len() as u32,
            )
        };
        self
    }
}
