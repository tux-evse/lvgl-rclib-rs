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
use std::any::Any;
use std::ffi::CString;
use std::mem;
use std::os::raw;
use std::{thread, time};

pub trait LvglCommon {
    fn get_handle(&self) -> *mut cglue::lv_obj_t;
    fn get_style(&self) -> *mut cglue::lv_style_t;
    fn get_uid(&self) -> &'static str;
    fn get_info(&self) -> &'static str;
    fn get_generic(&'static self) -> LvglWidget;
    fn get_callback(&self) -> Option<*mut dyn LvglHandler>;
    fn finalize(&'static self) -> &'static LvglWidget;
    fn callback(&self, _event: &cglue::lv_event_t) {}
    fn get_action(&self) -> &'static str {
        "[]"
    }
    fn set_callback(&'static self, ctrlbox: *mut dyn LvglHandler) -> &Self;
    fn set_info(&self, info: &'static str) -> &Self;
    fn as_any(&self) -> &dyn Any;
}
// common trait should be implemented for each widget because internal object struct is not identical
#[macro_export]
macro_rules! impl_widget_trait {
    ($widget:ty, $object:ident) => {
        impl LvglCommon for $widget {
            fn get_uid(&self) -> &'static str {
                self.uid
            }
            fn get_info(&self) -> &'static str {
                self.info.get()
            }
            fn set_info(&self, info: &'static str) -> &Self {
                self.info.set(info);
                self
            }
            fn get_handle(&self) -> *mut cglue::_lv_obj_t {
                self.handle
            }
            fn get_style(&self) -> *mut cglue::lv_style_t {
                self.style
            }
            fn get_callback(&self) -> Option<*mut dyn LvglHandler> {
                self.ctrlbox.get()
            }
            fn get_generic(&'static self) -> LvglWidget {
                LvglWidget::$object(self)
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            // if callback not set do it
            fn set_callback(&'static self, ctrlbox: *mut dyn LvglHandler) -> &Self {
                if let None = self.ctrlbox.get() {
                    self.ctrlbox.set(Some(ctrlbox));
                    let context = Box::leak(Box::new(LvglWidget::$object(self)));
                    unsafe {
                        cglue::lv_obj_add_event_cb(
                            self.get_handle(),
                            Some(lvgl_events_cb),
                            cglue::lv_event_code_t_LV_EVENT_ALL,
                            context as *const _ as *mut raw::c_void,
                        );
                    }
                }
                self
            }

            fn finalize(&'static self) -> &'static LvglWidget {
                Box::leak(Box::new(LvglWidget::$object(self)))
            }
        }
        impl LvglMethod for $widget {}
    };
}

pub trait LvglMethod {
    fn set_size(&self, width: i16, height: i16) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        unsafe {
            cglue::lv_obj_set_width(handle, width);
            cglue::lv_obj_set_height(handle, height);
        }
        self
    }

    fn set_width(&self, width: i16) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        unsafe {
            cglue::lv_obj_set_width(handle, width);
        }
        self
    }

    fn set_height(&self, height: i16) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        unsafe {
            cglue::lv_obj_set_height(handle, height);
        }
        self
    }
    fn set_color(&self, color: LvglColor) -> &Self
    where
        Self: LvglCommon,
    {
        let style = self.get_style();
        unsafe {
            cglue::lv_style_set_text_color(style, color.handle);
        }
        self
    }

    fn set_border(&self, width: i16, color: LvglColor) -> &Self
    where
        Self: LvglCommon,
    {
        let style = self.get_style();
        unsafe {
            cglue::lv_style_set_border_width(style, width);
            cglue::lv_style_set_border_color(style, color.handle);
        }
        self
    }

    fn set_padding(&self, top: i16, botton: i16, right: i16, left: i16) -> &Self
    where
        Self: LvglCommon,
    {
        let style = self.get_style();
        unsafe {
            cglue::lv_style_set_pad_top(style, top);
            cglue::lv_style_set_pad_bottom(style, botton);
            cglue::lv_style_set_pad_right(style, right);
            cglue::lv_style_set_pad_left(style, left);
        }
        self
    }

    fn set_disable(&self, lock: bool) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        unsafe {
            if lock {
                cglue::lv_obj_add_state(handle, cglue::LV_STATE_DISABLED as u16);
            } else {
                cglue::lv_obj_clear_state(handle, cglue::LV_STATE_DISABLED as u16);
            }
        }
        self
    }

    fn set_radius(&self) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        unsafe {
            cglue::lv_obj_set_style_radius(
                handle,
                cglue::LV_RADIUS_CIRCLE as i16,
                cglue::LV_STATE_DEFAULT,
            );
        }
        self
    }

    fn set_title(&self, text: &str, x_ofs: i16, y_ofs: i16, font: &LvglFont) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        unsafe {
            let title = cglue::lv_label_create(cglue::lv_scr_action());
            cglue::lv_obj_align_to(
                title,
                handle,
                cglue::LV_ALIGN_BOTTOM_LEFT as u8,
                x_ofs,
                y_ofs,
            );

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(title, style, 0);
            cglue::lv_obj_set_style_text_align(title, cglue::LV_TEXT_ALIGN_CENTER as u8, 0);

            cglue::lv_style_set_text_font(style, font as *const _ as *const cglue::lv_font_t);
            cglue::lv_obj_add_style(title, style, 0);
            let text = match CString::new(text) {
                Err(_) => CString::new("Non UTF8 label").unwrap(),
                Ok(value) => value,
            };
            cglue::lv_label_set_text(title, text.as_ptr());
        }
        self
    }

    fn set_background(&self, color: LvglColor) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        let style = self.get_style();
        unsafe {
            cglue::lv_style_set_bg_color(style, color.handle);
            cglue::lv_style_set_bg_opa(style, cglue::LV_OPA_50 as u8);
            cglue::lv_obj_add_style(handle, style, cglue::LV_STATE_DEFAULT);
        }
        self
    }

    fn get_states(&self) -> LvglStates
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        unsafe {
            LvglStates {
                handle: cglue::lv_obj_get_state(handle),
            }
        }
    }
}

pub struct LvglHandle {
    _disp_handle: *mut cglue::lv_disp_drv_t,
    _mouse_handle: *mut cglue::lv_indev_t,
}

impl LvglHandle {
    pub fn new(x_res: i16, y_res: i16, draw_ratio: u32) -> Self {
        unsafe {
            cglue::lv_init();

            #[cfg(not(use_gtk))]
            {
                cglue::fbdev_init();
                cglue::evdev_init();
            }
            #[cfg(use_gtk)]
            {
                cglue::gtkdrv_init();
            }

            // drawing buffer that can be smaller than screen definition
            let buffer_sz = x_res as u32 * y_res as u32 / draw_ratio;
            let disp_buffer = Vec::<cglue::lv_color_t>::with_capacity(buffer_sz as usize).leak();

            // draw buffer handle
            let draw_buffer = Box::leak(Box::new(mem::zeroed::<cglue::_lv_disp_draw_buf_t>()));
            cglue::lv_disp_draw_buf_init(
                draw_buffer,
                disp_buffer as *const _ as *mut raw::c_void,
                0 as *mut raw::c_void,
                buffer_sz as u32,
            );

            // frame buffer driver handle
            let disp_handle = Box::leak(Box::new(mem::zeroed::<cglue::lv_disp_drv_t>()));
            cglue::lv_disp_drv_init(disp_handle);
            disp_handle.draw_buf = draw_buffer;
            disp_handle.hor_res = x_res;
            disp_handle.ver_res = y_res;
            //disp_handle.physical_hor_res = x_res;
            //disp_handle.physical_ver_res = y_res;

            #[cfg(not(use_gtk))]
            {
                disp_handle.flush_cb = Some(cglue::fbdev_flush);
            }

            #[cfg(use_gtk)]
            {
                println!("--- GTK frame-buffer simulator selected ---");
                disp_handle.flush_cb = Some(cglue::gtkdrv_flush_cb);
            }
            cglue::lv_disp_drv_register(disp_handle);

            // input event handler
            let indev_handle = Box::leak(Box::new(mem::zeroed::<cglue::lv_indev_drv_t>()));
            indev_handle.type_ = cglue::lv_indev_type_t_LV_INDEV_TYPE_POINTER;

            #[cfg(not(use_gtk))]
            {
                indev_handle.read_cb = Some(cglue::evdev_read);
            }
            #[cfg(use_gtk)]
            {
                indev_handle.read_cb = Some(cglue::gtkdrv_mouse_read_cb);
            }

            let mouse_handle = cglue::lv_indev_drv_register(indev_handle);

            LvglHandle {
                _disp_handle: disp_handle,
                _mouse_handle: mouse_handle,
            }
        }
    }

    pub fn set_theme(
        &mut self,
        primary: LvglColor,
        secondary: LvglColor,
        dark: bool,
        font: &LvglFont,
    ) -> &Self {
        unsafe {
            let display = cglue::lv_disp_get_default();
            cglue::lv_disp_set_bg_color(display, cglue::lv_color_mk(100, 100, 100));
            cglue::lv_disp_set_bg_opa(display, 128);

            let theme = cglue::lv_theme_default_init(
                display,
                primary.handle,
                secondary.handle,
                dark,
                font as *const _ as *const cglue::lv_font_t,
            );
            cglue::lv_disp_set_theme(display as *mut cglue::_lv_disp_t, theme);
            /*
            let cursor_handle = cglue::lv_img_create(cglue::lv_scr_action());
            cglue::lv_img_set_src(
                cursor_handle,
                &cglue::lv_mouse_cursor as *const _ as *const raw::c_void,
            );
            cglue::lv_indev_set_cursor(self._mouse_handle, cursor_handle);
            */
        }
        self
    }

    pub fn get_root_widget(&self) -> &'static LvglWidget {
        &LvglWidget::Display()
    }

    // notify lvgl how long we've been sleeping update event and return next expected wait in ms
    pub fn start_loop(&self) {
        thread::spawn(|| {
            let mut tic = 5; // foOptione lvgl to process waiting events
            loop {
                unsafe {
                    cglue::lv_tick_inc(tic);
                    tic = cglue::lv_timer_handler()
                };
                let delay = time::Duration::from_millis(tic as u64);
                thread::sleep(delay);
            }
        });
    }
}
