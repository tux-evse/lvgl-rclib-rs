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
use std::any::Any;
use crate::prelude::*;

// exported cglue types
pub type LvglPoint  = cglue::lv_point_t;
pub type LvglImgDsc= cglue::lv_img_dsc_t;

// use only for test
#[allow(dead_code)]
pub(crate) const PRJ_DIR:&str = cglue::PRJ_DIR;

pub enum LvglWidget {
    ImgButton(&'static LvglImgButton),
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
}

pub trait LvglHandler {
    fn callback(&self, widget: &LvglWidget, uid: &'static str, event: &LvglEvent);
}

// has we share C widget callback, we have to retrieve initial object for callback
impl LvglWidget {
    pub(crate) fn callback(&self, event: &LvglEvent) {
        match self {
            LvglWidget::Button(this) => this.callback(self, event),
            LvglWidget::ImgButton(this) => this.callback(self, event),
            LvglWidget::PixButton(this) => this.callback(self, event),
            LvglWidget::Switch(this) => this.callback(self, event),
            _ => {}
        }
    }
    pub fn set_callback(&self, ctrlbox: *mut dyn LvglHandler) {
        match self {
            LvglWidget::Button(this) => this.set_callback(ctrlbox),
            LvglWidget::ImgButton(this) => this.set_callback(ctrlbox),
            LvglWidget::PixButton(this) => this.set_callback(ctrlbox),
            LvglWidget::Image(this) => this.set_callback(ctrlbox),
            LvglWidget::Switch(this) => this.set_callback(ctrlbox),
            _ => {}
        }
    }

    pub fn as_any(&self) -> &dyn Any {
        match self {
            LvglWidget::Label(this) => this.as_any(),
            LvglWidget::Button(this) => this.as_any(),
            LvglWidget::ImgButton(this) => this.as_any(),
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
        }
    }

    pub fn get_uid(&self) -> &'static str {
        match self {
            LvglWidget::Label(this) => this.get_uid(),
            LvglWidget::Button(this) => this.get_uid(),
            LvglWidget::ImgButton(this) => this.get_uid(),
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
        }
    }
}