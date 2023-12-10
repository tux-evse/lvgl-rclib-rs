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

// exported cglue types
pub type LvglPoint = cglue::lv_point_t;
pub type LvglImgDsc = cglue::lv_img_dsc_t;

// use only for test
#[allow(dead_code)]
pub(crate) const PRJ_DIR: &str = cglue::PRJ_DIR;

pub enum LvglWidget {
    Label(&'static LvglLabel),
    Button(&'static LvglButton),
    PixButton(&'static LvglPixButton),
    Pixmap(&'static LvglPixmap),
    TextArea(&'static LvglTextArea),
    Led(&'static LvglLed),
    Line(&'static LvglLine),
    Image(&'static LvglImage),
    Arc(&'static LvglArc),
    Meter(&'static LvglMeter),
    Switch(&'static LvglSwitch),
    Bar(&'static LvglBar),
    Qrcode(&'static LvglQrcode),
    Area(&'static LvglArea),
    Display(),
}

pub trait LvglHandler {
    fn callback(&self, widget: &LvglWidget, uid: &'static str, event: &LvglEvent);
}

// has we share C widget callback, we have to retrieve initial object for callback
impl LvglWidget {
    pub(crate) fn callback(&self, event: &LvglEvent) {
        match self {
            LvglWidget::Button(this) => this.callback(self, event),
            LvglWidget::PixButton(this) => this.callback(self, event),
            LvglWidget::Switch(this) => this.callback(self, event),
            _ => {}
        }
    }
    pub fn set_callback(&self, ctrlbox: *mut dyn LvglHandler) {
        match self {
            LvglWidget::Button(this) => {
                this.set_callback(ctrlbox);
            }
            LvglWidget::PixButton(this) => {
                this.set_callback(ctrlbox);
            }
            LvglWidget::Switch(this) => {
                this.set_callback(ctrlbox);
            }
            _ => {}
        }
    }

    pub fn get_handle(&self) -> *mut cglue::_lv_obj_t {
        match self {
            LvglWidget::Label(this) => this.get_handle(),
            LvglWidget::Button(this) => this.get_handle(),
            LvglWidget::Pixmap(this) => this.get_handle(),
            LvglWidget::TextArea(this) => this.get_handle(),
            LvglWidget::Led(this) => this.get_handle(),
            LvglWidget::Line(this) => this.get_handle(),
            LvglWidget::Image(this) => this.get_handle(),
            LvglWidget::Arc(this) => this.get_handle(),
            LvglWidget::Meter(this) => this.get_handle(),
            LvglWidget::Switch(this) => this.get_handle(),
            LvglWidget::Bar(this) => this.get_handle(),
            LvglWidget::Qrcode(this) => this.get_handle(),
            LvglWidget::PixButton(this) => this.get_handle(),
            LvglWidget::Area(this) => this.get_handle(),

            LvglWidget::Display() => unsafe { cglue::lv_scr_action() },
        }
    }

    pub fn as_any(&self) -> &dyn Any {
        match self {
            LvglWidget::Label(this) => this.as_any(),
            LvglWidget::Button(this) => this.as_any(),
            LvglWidget::Pixmap(this) => this.as_any(),
            LvglWidget::TextArea(this) => this.as_any(),
            LvglWidget::Led(this) => this.as_any(),
            LvglWidget::Line(this) => this.as_any(),
            LvglWidget::Image(this) => this.as_any(),
            LvglWidget::Arc(this) => this.as_any(),
            LvglWidget::Meter(this) => this.as_any(),
            LvglWidget::Switch(this) => this.as_any(),
            LvglWidget::Bar(this) => this.as_any(),
            LvglWidget::Qrcode(this) => this.as_any(),
            LvglWidget::PixButton(this) => this.as_any(),
            LvglWidget::Area(this) => this.as_any(),

            LvglWidget::Display() => &0 as &dyn Any,
        }
    }

    pub fn get_uid(&self) -> &'static str {
        match self {
            LvglWidget::Label(this) => this.get_uid(),
            LvglWidget::Button(this) => this.get_uid(),
            LvglWidget::Pixmap(this) => this.get_uid(),
            LvglWidget::TextArea(this) => this.get_uid(),
            LvglWidget::Led(this) => this.get_uid(),
            LvglWidget::Line(this) => this.get_uid(),
            LvglWidget::Image(this) => this.get_uid(),
            LvglWidget::Arc(this) => this.get_uid(),
            LvglWidget::Meter(this) => this.get_uid(),
            LvglWidget::Switch(this) => this.get_uid(),
            LvglWidget::Bar(this) => this.get_uid(),
            LvglWidget::Qrcode(this) => this.get_uid(),
            LvglWidget::PixButton(this) => this.get_uid(),
            LvglWidget::Area(this) => this.get_uid(),

            LvglWidget::Display() => "Root",
        }
    }
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

pub struct LvglStates {
    pub(crate)handle: cglue::lv_state_t,
}

#[allow(non_camel_case_types)]
pub enum LvglState {
    DEFAULT,
    CHECKED,
    FOCUSED,
    FOCUS_KEY,
    EDITED,
    HOVERED,
    PRESSED,
    SCROLLED,
    DISABLED,
    USER_1,
    USER_2,
    USER_3,
    USER_4,
    UNKNOWN,
}

impl LvglState {
    pub(crate) fn get_raw(&self) -> u16 {
        let value= match self {
            LvglState::DEFAULT => cglue::LV_STATE_DEFAULT,
            LvglState::CHECKED => cglue::LV_STATE_CHECKED,
            LvglState::FOCUSED => cglue::LV_STATE_FOCUSED,
            LvglState::FOCUS_KEY => cglue::LV_STATE_FOCUS_KEY,
            LvglState::EDITED => cglue::LV_STATE_EDITED,
            LvglState::HOVERED => cglue::LV_STATE_HOVERED,
            LvglState::PRESSED => cglue::LV_STATE_PRESSED,
            LvglState::SCROLLED => cglue::LV_STATE_SCROLLED,
            LvglState::DISABLED => cglue::LV_STATE_DISABLED,
            LvglState::USER_1 => cglue::LV_STATE_USER_1,
            LvglState::USER_2 => cglue::LV_STATE_USER_2,
            LvglState::USER_3 => cglue::LV_STATE_USER_3,
            LvglState::USER_4 => cglue::LV_STATE_USER_4,

            _ => 0,
        };
        value as u16
    }
}

impl LvglStates {
    pub fn check(&self, state: LvglState) -> bool {
        if (state.get_raw() & self.handle) != 0 {
            true
        } else {
            false
        }
    }
}
