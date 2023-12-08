/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Licensed under the Apache License, Version 2.0 (the "License"
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
use std::{thread, time};

pub fn display_init() -> LvglHandle {
    // Warning screen should have the same size as the one define into GTK emulator
    LvglHandle::new(1024, 600, 1)
}

pub fn display_loop() {
    let mut tic = 5; // foOptione lvgl to process waiting events
    loop {
        unsafe {
            cglue::lv_tick_inc(tic);
            tic = cglue::lv_timer_handler()
        };
        let delay = time::Duration::from_millis(tic as u64);
        thread::sleep(delay);
    }
}

struct WidgetEvtCtx {
    test: &'static str,
}

impl LvglHandler for WidgetEvtCtx {
    fn callback(&self, _widget: &LvglWidget, uid: &'static str, event: &LvglEvent) {
        println!(
            "Callback {{test:{} widget:{}, 'event':{:?}}}",
            self.test, uid, event
        );
    }
}

pub fn draw_date(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    let date= get_time("%D %H:%M");
    LvglLabel::new(root,"Local-Time", LvglMkFont::std_14(), x_ofs, y_ofs)
    .set_value(date.unwrap().as_str());
}

pub fn draw_label(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    LvglLabel::new(root,"Label-1", LvglMkFont::std_22(), x_ofs, y_ofs)
        .set_value("Demo Label widget")
        .set_title("Label widget", 100, 75, LvglMkFont::std_10())
        .set_size(300, 100)
        .set_color(LvglColor::rvb(0, 0, 0))
        .set_background(LvglColor::rvb(0xFF, 0xFF, 0xFF))
        .set_border(3, LvglColor::rvb(0, 0xff, 0));
}

pub fn draw_icon(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    LvglPixmap::new(root,"Icon-wifi", LvglIcon::WIFI, x_ofs, y_ofs).set_info("Demo Wifi Icon");
    LvglPixmap::new(root,"Icon-Nfc", LvglIcon::SD_CARD, x_ofs + 25, y_ofs)
        .set_color(LvglColor::rvb(255, 0, 0));
    LvglPixmap::new(root,"Icon-Battery", LvglIcon::BATTERY_2, x_ofs + 50, y_ofs);
}

pub fn draw_led(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    LvglLed::new(root,"Led-Red", x_ofs, y_ofs)
        .set_info("red led")
        .set_color(LvglColor::RED())
        .set_size(10, 10)
        .set_on(true);

    LvglLed::new(root,"Led-Green", x_ofs + 25, y_ofs)
        .set_color(LvglColor::rvb(0, 255, 0))
        .set_info("green led")
        .set_brightness(255)
        .set_size(10, 10)
        .set_on(true);
}

pub fn draw_text(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    LvglTextArea::new(root,"Text-Area", x_ofs, y_ofs)
        .set_info("Demo Text area Zone")
        .set_width(600)
        .set_value("display message zone");
}

pub fn draw_switch(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    LvglSwitch::new(root,"Switch-1", x_ofs, y_ofs)
        .set_check(false)
        .set_height(20);

    LvglSwitch::new(root,"Switch-2", x_ofs + 75, y_ofs)
        .set_check(true)
        .set_height(20);
}

pub fn draw_line(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    let points = [
        LvglPoint { x: 5, y: 5 },
        LvglPoint { x: 70, y: 70 },
        LvglPoint { x: 120, y: 10 },
        LvglPoint { x: 180, y: 60 },
        LvglPoint { x: 240, y: 10 },
    ];

    LvglLine::new(root,"Line", x_ofs, y_ofs)
        .set_color(LvglColor::RED())
        .set_width(8)
        .set_rounded(true)
        .set_points(Box::new(points));
}

pub fn draw_button(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    LvglButton::new(root,"Button-A", LvglMkFont::std_18(), x_ofs, y_ofs)
        .set_value("Test-1")
        .set_size(180, 100)
        .set_border(3, LvglColor::DEEP_ORANGE())
        .set_callback(Box::leak(Box::new(WidgetEvtCtx {
            test: "Draw-Button-1",
        })));

    LvglButton::new(root,"Button-B", LvglMkFont::std_14(), x_ofs + 50, y_ofs+110)
        .set_value("Test-2")
        .set_callback(Box::leak(Box::new(WidgetEvtCtx {
            test: "Draw-Button-2",
        })));

    LvglPixButton::new(root,"Button-Img",  x_ofs+50, y_ofs-60)
        .set_value( LvglIcon::HOME)
        .set_background(LvglColor::BLUE_GREY())
        .set_title("Clickable", 8, 6, LvglMkFont::std_10())
        .set_border(3, LvglColor::DEEP_PURPLE())
        .set_callback(Box::leak(Box::new(WidgetEvtCtx {
            test: "Draw-PixButton",
        })));

}

pub fn draw_arc(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    LvglArc::new(root,"Arc", 0, 300, x_ofs, y_ofs)
        .set_info("Arc widget")
        .set_value(180);
}

pub fn draw_tux(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    let tux_path = PRJ_DIR.to_string() + "/assets/tux-evsex250.png";
    LvglImage::new(root,"tux-evse", tux_path.as_str(), x_ofs, y_ofs)
    .set_title("tux-evse mascot", 65, -20, LvglMkFont::std_14());
}

pub fn draw_qrcode(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    LvglQrcode::new(root,
        "qr-code",
        LvglColor::LIGHT_BLUE(),
        LvglColor::DEEP_PURPLE(),
        150,
        x_ofs,
        y_ofs,
    )
    .set_value("https://github.com/tux-evse")
    .set_title("tux-evse@github", 10, 0, LvglMkFont::std_14());
}

pub fn draw_bar(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    LvglBar::new(root,"Bar-1", 10, 90, x_ofs, y_ofs)
        .set_info("variable bar")
        .set_size(10, 250)
        .set_gradient(
            true,
            LvglColor::GREEN(),
            LvglColor::YELLOW(),
        )
        .set_value(60);

    LvglBar::new(root,"Bar-2", 10, 90, x_ofs, y_ofs - 30)
        .set_info("variable bar")
        .set_size(250, 10)
        .set_gradient(
            false,
            LvglColor::GREEN(),
            LvglColor::YELLOW(),
        )
        .set_value(40);
}

pub fn draw_meter(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    LvglMeter::new(root,
        "Meter",
        4,
        -10,
        LvglColor::INDIGO(),
        x_ofs,
        y_ofs,
    )
    .set_size(200, 200)
    .set_tic(
        3,
        10,
        41,
        10,
        8,
        LvglColor::BLUE_GREY(),
        LvglColor::GREY(),
    )
    .set_zone(0, 20, 4, LvglColor::RED())
    .set_zone(80, 100, 4, LvglColor::GREEN())
    .set_border(4, LvglColor::LIGHT_BLUE())
    .set_background(LvglColor::PINK())
    .set_value(50);
}

pub fn draw_area(root: &LvglWidget,x_ofs: i16, y_ofs: i16) {
    let area= LvglArea::new(root, "Area", x_ofs, y_ofs)
    .set_size (200,200)
    .finalize();

    LvglBar::new(area,"Bar-1", 10, 90, 10, 10)
        .set_info("variable bar")
        .set_size(10, 250)
        .set_gradient(
            true,
            LvglColor::GREEN(),
            LvglColor::YELLOW(),
        )
        .set_value(60);

    LvglArc::new(area,"Arc", 0, 300, 100, 100)
        .set_info("Arc widget")
        .set_value(180);
}

#[test]
pub fn test_label() {
    let root= display_init().get_root_widget();
    draw_label(root, 100, 100);
    display_loop();
}

#[test]
pub fn test_date() {
    let root= display_init().get_root_widget();
    draw_date(root,100, 100);
    display_loop();
}


#[test]
pub fn test_icon() {
    let root= display_init().get_root_widget();
    draw_icon(root,900, 10);
    display_loop();
}

#[test]
pub fn test_led() {
    let root= display_init().get_root_widget();
    draw_led(root,100, 100);
    display_loop();
}

#[test]
pub fn test_text() {
    let root= display_init().get_root_widget();
    draw_text(root,100, 100);
    display_loop();
}

#[test]
pub fn test_switch() {
    let root= display_init().get_root_widget();
    draw_switch(root,100, 100);
    display_loop();
}
#[test]
pub fn test_line() {
    let root= display_init().get_root_widget();
    draw_line(root,100, 100);
    display_loop();
}
#[test]
pub fn test_button() {
    let root= display_init().get_root_widget();
    draw_button(root,100, 100);
    display_loop();
}
#[test]
pub fn test_arc() {
    let root= display_init().get_root_widget();
    draw_arc(root,100, 100);
    display_loop();
}
#[test]
pub fn test_bar() {
    let root= display_init().get_root_widget();
    draw_bar(root,100, 100);
    display_loop();
}

#[test]
pub fn test_tux() {
    let root= display_init().get_root_widget();
    draw_tux(root,100, 100);
    display_loop();
}

#[test]
pub fn test_qrcode() {
    let root= display_init().get_root_widget();
    draw_qrcode(root,100, 100);
    display_loop();
}

#[test]
pub fn test_meter() {
    let root= display_init().get_root_widget();
    draw_meter(root,100, 100);
    display_loop();
}

#[test]
pub fn test_area() {
    let root= display_init().get_root_widget();
    draw_area(root,100, 100);
    display_loop();
}

#[test]
pub fn test_pannel() {
    let primary = LvglColor::LIGHT_BLUE();
    let secondary = LvglColor::BLUE_GREY();
    let mut display = display_init();
    display.set_theme(primary, secondary, false, LvglMkFont::std_14());
    let root= display.get_root_widget();
    draw_icon(root,900, 10);
    draw_led(root,850, 10);
    draw_date(root,540, 12);
    draw_switch(root,650, 10);
    draw_line(root,400, 70);
    draw_button(root,450, 200);
    draw_arc(root,100, 30);
    draw_bar(root,100, 250);
    draw_meter(root,800, 350);
    draw_label(root,240, 400);
    draw_tux(root,765, 100);
    draw_qrcode(root,600, 370);
    draw_text(root,50, 550);
    display_loop();
}
