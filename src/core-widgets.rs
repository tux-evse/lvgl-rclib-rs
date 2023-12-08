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
use crate::prelude::*;
use std::any::Any;
use std::cell::Cell;
use std::ffi::CString;
use std::mem;
use std::os::raw;

pub struct LvglButton {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    label: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}

impl_widget_trait!(LvglButton, Button);
impl LvglButton {
    pub fn new(
        parent: &LvglWidget,
        uid: &'static str,
        font: &LvglFont,
        x_ofs: i16,
        y_ofs: i16,
    ) -> &'static Self {
        unsafe {
            let handle = cglue::lv_btn_create(parent.get_handle());
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            let button = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(button);
            cglue::lv_style_set_text_font(button, font as *const _ as *const cglue::lv_font_t);
            cglue::lv_obj_add_style(handle, button, 0);

            // create a label as children on button
            let label = cglue::lv_label_create(handle);
            cglue::lv_obj_set_align(label, cglue::LV_TEXT_ALIGN_CENTER as u8);
            cglue::lv_obj_set_pos(label, 0, 0);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglButton {
                uid,
                info: Cell::new(""),
                handle,
                style,
                label,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_circular(&self) -> &Self {
        unsafe {
            cglue::lv_label_set_long_mode(self.handle, cglue::LV_LABEL_LONG_SCROLL_CIRCULAR as u8);
        }
        self
    }

    pub fn set_value(&self, label: &str) -> &Self {
        unsafe {
            let text = match CString::new(label) {
                Err(_) => CString::new("Non UTF8 label").unwrap(),
                Ok(value) => value,
            };
            cglue::lv_label_set_text(self.label, text.as_ptr());
        }
        self
    }

    pub fn callback(&self, widget: &LvglWidget, event: &LvglEvent) {
        if let Some(ctrlbox) = self.ctrlbox.get() {
            match event {
                LvglEvent::PRESSED => {}
                //LvglEvent::CLICKED => {}
                _ => return, // ignore other event
            }
            unsafe { (*ctrlbox).callback(widget, self.uid, event) };
        }
    }
}

pub struct LvglLabel {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglLabel, Label);
impl LvglLabel {
    pub fn new(
        parent: &LvglWidget,
        uid: &'static str,
        font: &LvglFont,
        x_ofs: i16,
        y_ofs: i16,
    ) -> &'static Self {
        unsafe {
            let handle = cglue::lv_label_create(parent.get_handle());
            cglue::lv_label_set_recolor(handle, false);
            cglue::lv_obj_set_style_text_align(handle, cglue::LV_TEXT_ALIGN_CENTER as u8, 0);
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            let label = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(label);
            cglue::lv_style_set_text_font(label, font as *const _ as *const cglue::lv_font_t);
            cglue::lv_obj_add_style(handle, label, 0);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglLabel {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_circular(&self) -> &Self {
        unsafe {
            cglue::lv_label_set_long_mode(self.handle, cglue::LV_LABEL_LONG_SCROLL_CIRCULAR as u8);
        }
        self
    }

    pub fn set_value(&self, text: &str) -> &Self {
        unsafe {
            let text = match CString::new(text) {
                Err(_) => CString::new("Non UTF8 label").unwrap(),
                Ok(value) => value,
            };
            cglue::lv_label_set_text(self.handle, text.as_ptr());
        }
        self
    }
}

pub struct LvglPixButton {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    image: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}

impl_widget_trait!(LvglPixButton, PixButton);
impl LvglPixButton {
    pub fn new(parent: &LvglWidget, uid: &'static str, x_ofs: i16, y_ofs: i16) -> &'static Self {
        unsafe {
            let handle = cglue::lv_btn_create(parent.get_handle());
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            let image = cglue::lv_img_create(handle);
            cglue::lv_obj_align(image, cglue::LV_ALIGN_CENTER as u8, 0, 0);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglPixButton {
                uid,
                info: Cell::new(""),
                handle,
                style,
                image,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_value<T>(&self, pixmap: T) -> &Self
    where
        LvglPixmap: ImgToVoid<T>,
    {
        let imgref = LvglPixmap::get_ref(pixmap);
        unsafe {
            cglue::lv_img_set_src(self.image, imgref);
        }
        self
    }

    pub fn get_action(&self) -> &'static str {
        &"['ON','OFF']"
    }

    pub fn callback(&self, widget: &LvglWidget, event: &LvglEvent) {
        if let Some(ctrlbox) = self.ctrlbox.get() {
            match event {
                LvglEvent::PRESSED => {}
                //LvglEvent::CLICKED => {}
                _ => return, // ignore other event
            }
            unsafe { (*ctrlbox).callback(widget, self.uid, event) };
        }
    }
}

pub trait ImgToVoid<T> {
    fn get_ref(image: T) -> *mut raw::c_void;
}

impl ImgToVoid<&[u8; 4]> for LvglPixmap {
    fn get_ref(image: &[u8; 4]) -> *mut raw::c_void {
        image as *const _ as *mut raw::c_void
    }
}

impl ImgToVoid<&LvglImgDsc> for LvglPixmap {
    fn get_ref(image: &LvglImgDsc) -> *mut raw::c_void {
        image as *const _ as *mut raw::c_void
    }
}

pub struct LvglPixmap {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}

impl_widget_trait!(LvglPixmap, Pixmap);
impl LvglPixmap {
    pub fn new<T>(
        parent: &LvglWidget,
        uid: &'static str,
        pixmap: T,
        x_ofs: i16,
        y_ofs: i16,
    ) -> &'static Self
    where
        Self: ImgToVoid<T>,
    {
        let imgref = LvglPixmap::get_ref(pixmap);
        unsafe {
            let handle = cglue::lv_img_create(parent.get_handle());
            cglue::lv_img_set_src(handle, imgref);
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglPixmap {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn get_action(&self) -> &'static str {
        &"['ON','OFF']"
    }

    pub fn set_value<T>(&self, pixmap: T) -> &Self
    where
        Self: ImgToVoid<T>,
    {
        let imgref = LvglPixmap::get_ref(pixmap);
        unsafe {
            cglue::lv_img_set_src(self.handle, imgref);
        }
        self
    }

    pub fn set_angle(&self, rotation: i16) -> &Self {
        unsafe {
            cglue::lv_img_set_angle(self.handle, rotation);
        }
        self
    }

    pub fn set_zoom(&self, zoom: u16) -> &Self {
        unsafe {
            cglue::lv_img_set_zoom(self.handle, zoom);
        }
        self
    }
}

pub struct LvglImage {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglImage, Image);
impl LvglImage {
    pub fn new(
        parent: &LvglWidget,
        uid: &'static str,
        path: &str,
        x_ofs: i16,
        y_ofs: i16,
    ) -> &'static Self {
        let mut img_path = path.to_string();
        img_path.insert_str(0, "L:"); // ugly lvgl path pattern
        let filepath = match CString::new(img_path) {
            Err(_) => CString::new("Non UTF8 path").unwrap(),
            Ok(value) => value,
        };

        unsafe {
            let handle = cglue::lv_img_create(parent.get_handle());
            cglue::lv_img_set_src(handle, filepath.as_ptr() as *mut raw::c_void);
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglImage {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }
    pub fn set_value<T>(&self, path: &str) -> &Self {
        let mut img_path = path.to_string();
        img_path.insert_str(0, "L:"); // ugly lvgl path pattern
        let filepath = match CString::new(img_path) {
            Err(_) => CString::new("Non UTF8 path").unwrap(),
            Ok(value) => value,
        };

        unsafe {
            cglue::lv_img_set_src(self.handle, filepath.as_ptr() as *mut raw::c_void);
        }
        self
    }

    pub fn set_angle(&self, rotation: i16) -> &Self {
        unsafe {
            cglue::lv_img_set_angle(self.handle, rotation);
        }
        self
    }

    pub fn set_zoom(&self, zoom: u16) -> &Self {
        unsafe {
            cglue::lv_img_set_zoom(self.handle, zoom);
        }
        self
    }
}

pub struct LvglTextArea {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglTextArea, TextArea);
impl LvglTextArea {
    pub fn new(parent: &LvglWidget, uid: &'static str, x_ofs: i16, y_ofs: i16) -> &'static Self {
        unsafe {
            let handle = cglue::lv_textarea_create(parent.get_handle());
            cglue::lv_obj_set_style_text_align(handle, cglue::LV_TEXT_ALIGN_CENTER as u8, 0);
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);
            cglue::lv_textarea_set_one_line(handle, true);
            cglue::lv_textarea_set_align(handle, cglue::LV_TEXT_ALIGN_LEFT as u8);
            cglue::lv_obj_set_style_text_opa(
                handle,
                cglue::LV_PART_MAIN as u8,
                cglue::LV_STATE_FOCUSED,
            );

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglTextArea {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_value(&self, text: &str) -> &Self {
        unsafe {
            let text = match CString::new(text) {
                Err(_) => CString::new("Non UTF8 text").unwrap(),
                Ok(value) => value,
            };
            cglue::lv_textarea_set_text(self.handle, text.as_ptr());
        }
        self
    }

    pub fn insert_text(&self, text: &str) -> &Self {
        unsafe {
            let text = match CString::new(text) {
                Err(_) => CString::new("Non UTF8 text").unwrap(),
                Ok(value) => value,
            };
            cglue::lv_textarea_set_text(self.handle, text.as_ptr());
        }
        self
    }
}

pub struct LvglLed {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglLed, Led);
impl LvglLed {
    pub fn new(parent: &LvglWidget, uid: &'static str, x_ofs: i16, y_ofs: i16) -> &'static Self {
        unsafe {
            let handle = cglue::lv_led_create(parent.get_handle());
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);
            cglue::lv_led_off(handle);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglLed {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn callback(&self, widget: &LvglWidget, event: &LvglEvent) {
        if let Some(ctrlbox) = self.ctrlbox.get() {
            match event {
                LvglEvent::PRESSED => {}
                _ => return, // ignore other events
            }
            unsafe { (*ctrlbox).callback(widget, self.uid, event) };
        }
    }

    pub fn set_color(&self, color: LvglColor) -> &Self {
        unsafe { cglue::lv_led_set_color(self.handle, color.handle) };
        self
    }

    pub fn set_brightness(&self, bright: u8) -> &Self {
        unsafe { cglue::lv_led_set_brightness(self.handle, bright) };
        self
    }

    pub fn get_action(&self) -> &'static str {
        &"['ON','OFF']"
    }

    pub fn set_on(&self, status: bool) -> &Self {
        unsafe {
            if status {
                cglue::lv_led_on(self.handle);
            } else {
                cglue::lv_led_off(self.handle);
            }
        }
        self
    }
}

pub struct LvglLine {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglLine, Line);
impl LvglLine {
    pub fn new(parent: &LvglWidget, uid: &'static str, x_ofs: i16, y_ofs: i16) -> &'static Self {
        unsafe {
            let handle = cglue::lv_line_create(parent.get_handle());
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglLine {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_points(&self, points: Box<[LvglPoint]>) -> &Self {
        let list = Box::leak(points);
        let count = list.len();
        unsafe {
            cglue::lv_line_set_points(
                self.handle,
                list.as_ptr() as *const cglue::lv_point_t,
                count as u16,
            );
        }
        self
    }

    pub fn set_width(&self, width: i16) -> &Self {
        unsafe {
            cglue::lv_style_set_line_width(self.style, width);
        }
        self
    }

    pub fn set_color(&self, color: LvglColor) -> &Self {
        unsafe {
            cglue::lv_style_set_line_color(self.style, color.handle);
        };
        self
    }

    pub fn set_rounded(&self, value: bool) -> &Self {
        unsafe {
            cglue::lv_style_set_line_rounded(self.style, value);
        };
        self
    }
}

pub struct LvglArc {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglArc, Arc);
impl LvglArc {
    pub fn new(
        parent: &LvglWidget,
        uid: &'static str,
        angle_start: u16,
        angle_end: u16,
        x_ofs: i16,
        y_ofs: i16,
    ) -> &'static Self {
        unsafe {
            let handle = cglue::lv_arc_create(parent.get_handle());
            cglue::lv_arc_set_bg_angles(handle, angle_start, angle_end);
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);
            cglue::lv_obj_clear_flag(handle, cglue::LV_OBJ_FLAG_CLICKABLE);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglArc {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_rotation(&self, angle: u16) -> &Self {
        unsafe {
            cglue::lv_arc_set_rotation(self.handle, angle);
        }
        self
    }

    pub fn set_range(&self, min: i16, max: i16) -> &Self {
        unsafe {
            cglue::lv_arc_set_range(self.handle, min, max);
        }
        self
    }

    pub fn set_value(&self, value: i32) -> &Self {
        unsafe {
            cglue::lv_arc_set_value(self.handle, value as i16);
        }
        self
    }

    pub fn remove_knob(&self) -> &Self {
        unsafe {
            cglue::lv_obj_remove_style(
                self.handle,
                0 as *mut cglue::lv_style_t,
                cglue::LV_PART_KNOB,
            );
        }
        self
    }

    pub fn set_width(&self, width: i16) -> &Self {
        unsafe {
            cglue::lv_style_set_arc_width(self.style, width);
        }
        self
    }

    pub fn set_color(&self, color: LvglColor) -> &Self {
        unsafe {
            cglue::lv_style_set_arc_color(self.style, color.handle);
        };
        self
    }

    pub fn set_rounded(&self, value: bool) -> &Self {
        unsafe {
            cglue::lv_style_set_arc_rounded(self.style, value);
        };
        self
    }
}

pub struct LvglSwitch {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglSwitch, Switch);
impl LvglSwitch {
    pub fn new(parent: &LvglWidget, uid: &'static str, x_ofs: i16, y_ofs: i16) -> &'static Self {
        unsafe {
            let handle = cglue::lv_switch_create(parent.get_handle());
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);
            cglue::lv_obj_add_state(handle, cglue::LV_STATE_CHECKED as u16);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglSwitch {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn get_action(&self) -> &'static str {
        &"['ON','OFF']"
    }

    pub fn set_value(&self, on: bool) -> &Self {
        unsafe {
            if on {
                cglue::lv_obj_add_state(self.handle, cglue::LV_STATE_CHECKED as u16);
            } else {
                cglue::lv_obj_clear_state(self.handle, cglue::LV_STATE_CHECKED as u16);
            }
        }
        self
    }

    pub fn set_lock(&self, lock: bool) -> &Self {
        unsafe {
            if lock {
                cglue::lv_obj_add_state(self.handle, cglue::LV_STATE_DISABLED as u16);
            } else {
                cglue::lv_obj_clear_state(self.handle, cglue::LV_STATE_DISABLED as u16);
            }
        }
        self
    }
    pub fn callback(&self, widget: &LvglWidget, event: &LvglEvent) {
        if let Some(ctrlbox) = self.ctrlbox.get() {
            match event {
                LvglEvent::VALUE_CHANGED => {}
                _ => return, // ignore other events
            }
            unsafe { (*ctrlbox).callback(widget, self.uid, event) };
        }
    }
}

pub struct LvglBar {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglBar, Bar);
impl LvglBar {
    pub fn new(
        parent: &LvglWidget,
        uid: &'static str,
        min: i32,
        max: i32,
        x_ofs: i16,
        y_ofs: i16,
    ) -> &'static Self {
        unsafe {
            let handle = cglue::lv_bar_create(parent.get_handle());
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);
            cglue::lv_bar_set_range(handle, min, max);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglBar {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_gradient(&self, vertical: bool, color: LvglColor, background: LvglColor) -> &Self {
        unsafe {
            cglue::lv_style_set_bg_opa(self.style, cglue::LV_OPA_COVER as u8);
            cglue::lv_style_set_bg_color(self.style, background.handle);
            cglue::lv_style_set_bg_grad_color(self.style, color.handle);
            if vertical {
                cglue::lv_style_set_bg_grad_dir(self.style, cglue::LV_GRAD_DIR_VER as u8);
            } else {
                cglue::lv_style_set_bg_grad_dir(self.style, cglue::LV_GRAD_DIR_HOR as u8);
            }
            cglue::lv_obj_add_style(self.handle, self.style, cglue::LV_PART_INDICATOR);
        }
        self
    }

    pub fn set_value(&self, value: i32) -> &Self {
        unsafe {
            cglue::lv_bar_set_value(
                self.handle,
                value,
                cglue::lv_anim_enable_t_LV_ANIM_OFF as u32,
            );
        }
        self
    }

    pub fn callback(&self, widget: &LvglWidget, event: &LvglEvent) {
        if let Some(ctrlbox) = self.ctrlbox.get() {
            match event {
                LvglEvent::VALUE_CHANGED => {}
                _ => return, // ignore other events
            }
            unsafe { (*ctrlbox).callback(widget, self.uid, event) };
        }
    }
}

pub struct LvglArea {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglArea, Area);
impl LvglArea {
    pub fn new(parent: &LvglWidget, uid: &'static str, x_ofs: i16, y_ofs: i16) -> &'static Self {
        unsafe {
            let handle = cglue::lv_obj_create(parent.get_handle());
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglArea {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }
}
