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

struct ButtonEvtCtx {
    app_data: &'static str,
}

impl LvglHandler for ButtonEvtCtx {
    fn callback(&self, widget: &LvglWidget, uid: &'static str, event: &LvglEvent) {
        // ignore any non button widget (should never happen)
        let _button = match widget.as_any().downcast_ref::<LvglButton>() {
            Some(widget) => widget,
            None => return,
        };

        println!(
            "Button-Callback {{app_data:{} widget:{}, 'event':{:?}}}",
            self.app_data, uid, event,
        );
    }
}

struct PixmapEvtCtx {
    app_data: &'static str,
}

impl LvglHandler for PixmapEvtCtx {
    fn callback(&self, widget: &LvglWidget, uid: &'static str, event: &LvglEvent) {
        let _pixmap = match widget.as_any().downcast_ref::<LvglPixButton>() {
            Some(widget) => widget,
            None => return,
        };
        println!(
            "Pixmap-Callback {{app_data:{} widget:{}, 'event':{:?}}}",
            self.app_data, uid, event
        );
    }
}

struct SwitchEvtCtx {
    app_data: &'static str,
}
impl LvglHandler for SwitchEvtCtx {
    fn callback(&self, widget: &LvglWidget, uid: &'static str, event: &LvglEvent) {
        let switch = match widget.as_any().downcast_ref::<LvglSwitch>() {
            Some(widget) => widget,
            None => return,
        };

        let states = switch.get_states();
        println!(
            "Switch-Callback {{app_data:{} widget:{}, 'event':{:?} 'checked':{}}}",
            self.app_data,
            uid,
            event,
            states.check(LvglState::CHECKED)
        );
    }
}

pub fn draw_date(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    let date = get_time("%D %H:%M");
    LvglLabel::new(root, "Local-Time", LvglMkFont::std_14(), x_ofs, y_ofs)
        .set_value(date.unwrap().as_str());
}

pub fn draw_label(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    LvglLabel::new(root, "Label-1", LvglMkFont::std_22(), x_ofs, y_ofs)
        .set_value("Demo Label widget")
        .set_title("Label widget", 100, 75, LvglMkFont::std_10())
        .set_size(300, 100)
        .set_color(LvglColor::rvb(0, 0, 0))
        .set_background(LvglColor::rvb(0xFF, 0xFF, 0xFF))
        .set_border(3, LvglColor::rvb(0, 0xff, 0));
}

pub fn draw_icon(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    let area = LvglArea::new(root, "Icon-Area", x_ofs, y_ofs)
        .set_size(80, 40)
        .set_padding(5, 2, 5, 5)
        .set_title("icon area", 3, 5, LvglMkFont::std_10())
        .finalize();

    LvglPixmap::new(area, "Icon-wifi", LvglIcon::WIFI, 10, 0).set_info("Demo Wifi Icon");
    LvglPixmap::new(area, "Icon-Nfc", LvglIcon::SD_CARD, 35, 0)
        .set_color(LvglColor::rvb(255, 0, 0));
    LvglPixmap::new(root, "Icon-Battery", LvglIcon::BATTERY_2, 50, 0);
}

pub fn draw_led(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    let area = LvglArea::new(root, "Led-Area", x_ofs, y_ofs)
        .set_size(70, 40)
        .set_padding(10, 2, 5, 5)
        .set_title("led area", 7, 7, LvglMkFont::std_10())
        .finalize();

    LvglLed::new(area, "Led-Red", 10, 0)
        .set_info("red led")
        .set_color(LvglColor::RED())
        .set_size(10, 10)
        .set_on(true);

    LvglLed::new(area, "Led-Green", 35, 0)
        .set_color(LvglColor::rvb(0, 255, 0))
        .set_info("green led")
        .set_brightness(255)
        .set_size(10, 10)
        .set_on(true);
}

pub fn draw_switch(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    let area = LvglArea::new(root, "Switch-Area", x_ofs, y_ofs)
        .set_size(150, 50)
        .set_padding(5, 2, 5, 10)
        .set_title("switch area", 35, 5, LvglMkFont::std_10())
        .finalize();

    LvglSwitch::new(area, "Switch-1", 0, 0)
        .set_title("Unlock", 3, 5, LvglMkFont::std_10())
        .set_disable(false)
        .set_value(false)
        .set_callback(Box::leak(Box::new(SwitchEvtCtx {
            app_data: "Draw-Button-1",
        })))
        .set_height(20)
        .finalize();

    LvglSwitch::new(area, "Switch-2", 75, 0)
        .set_title("Locked", 3, 5, LvglMkFont::std_10())
        .set_disable(true)
        .set_value(true)
        .set_callback(Box::leak(Box::new(SwitchEvtCtx {
            app_data: "Draw-Button-1",
        })))
        .set_height(20)
        .finalize();
}

pub fn draw_text(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    LvglTextArea::new(root, "Text-Area", x_ofs, y_ofs)
        .set_info("Demo Text area Zone")
        .set_width(600)
        .set_value("display message zone")
        .finalize();
}

pub fn draw_line(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    let points = [
        LvglPoint { x: 5, y: 5 },
        LvglPoint { x: 70, y: 70 },
        LvglPoint { x: 120, y: 10 },
        LvglPoint { x: 180, y: 60 },
        LvglPoint { x: 240, y: 10 },
    ];

    LvglLine::new(root, "Line", x_ofs, y_ofs)
        .set_color(LvglColor::RED())
        .set_width(8)
        .set_rounded(true)
        .set_points(Box::new(points));
}

pub fn draw_button(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    LvglButton::new(root, "Button-A", LvglMkFont::std_18(), x_ofs, y_ofs)
        .set_value("Test-1")
        .set_size(180, 100)
        .set_border(3, LvglColor::DEEP_ORANGE())
        .set_callback(Box::leak(Box::new(ButtonEvtCtx {
            app_data: "Draw-Button-1",
        })))
        .finalize();

    LvglButton::new(
        root,
        "Button-B",
        LvglMkFont::std_14(),
        x_ofs + 50,
        y_ofs + 110,
    )
    .set_value("Test-2")
    .set_callback(Box::leak(Box::new(ButtonEvtCtx {
        app_data: "Draw-Button-2",
    })))
    .finalize();

    LvglPixButton::new(root, "Button-Img", x_ofs + 50, y_ofs - 60)
        .set_value(LvglIcon::HOME)
        .set_background(LvglColor::BLUE_GREY())
        .set_title("Clickable", -13, 25, LvglMkFont::std_10())
        .set_border(3, LvglColor::DEEP_PURPLE())
        .set_callback(Box::leak(Box::new(PixmapEvtCtx {
            app_data: "Draw-PixButton",
        })))
        .finalize();
}

pub fn draw_arc(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    LvglArc::new(root, "Arc", 0, 300, x_ofs, y_ofs)
        .set_info("Arc widget")
        .set_value(180)
        .finalize();
}

pub fn draw_tux(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    let tux_path = PRJ_DIR.to_string() + "/assets/tux-evsex250.png";
    LvglImage::new(root, "tux-evse", tux_path.as_str(), x_ofs, y_ofs).set_title(
        "tux-evse mascot",
        65,
        0,
        LvglMkFont::std_14(),
    );
}

pub fn draw_qrcode(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    LvglQrcode::new(
        root,
        "qr-code",
        LvglColor::LIGHT_BLUE(),
        LvglColor::DEEP_PURPLE(),
        150,
        x_ofs,
        y_ofs,
    )
    .set_value("https://github.com/tux-evse")
    .set_title("tux-evse@github", 15, 15, LvglMkFont::std_14())
    .finalize();
}

pub fn draw_bar(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    LvglBar::new(root, "Bar-1", 10, 90, x_ofs, y_ofs)
        .set_info("variable bar")
        .set_size(10, 250)
        .set_gradient(true, LvglColor::GREEN(), LvglColor::YELLOW())
        .set_value(60)
        .finalize();

    LvglBar::new(root, "Bar-2", 10, 90, x_ofs, y_ofs - 30)
        .set_info("variable bar")
        .set_size(250, 10)
        .set_gradient(false, LvglColor::GREEN(), LvglColor::YELLOW())
        .set_value(40)
        .finalize();
}

pub fn draw_meter(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    LvglMeter::new(root, "Meter", 4, -10, LvglColor::INDIGO(), x_ofs, y_ofs)
        .set_size(200, 200)
        .set_tic(3, 10, 41, 10, 8, LvglColor::BLUE_GREY(), LvglColor::GREY())
        .set_zone(0, 20, 4, LvglColor::RED())
        .set_zone(80, 100, 4, LvglColor::GREEN())
        .set_border(4, LvglColor::LIGHT_BLUE())
        .set_background(LvglColor::PINK())
        .set_value(50)
        .finalize();
}

pub fn draw_area(root: &LvglWidget, x_ofs: i16, y_ofs: i16) {
    let area = LvglArea::new(root, "Area", x_ofs, y_ofs)
        .set_size(200, 200)
        .finalize();

    LvglBar::new(area, "Bar-1", 10, 90, 10, 10)
        .set_info("variable bar")
        .set_size(10, 250)
        .set_gradient(true, LvglColor::GREEN(), LvglColor::YELLOW())
        .set_value(60)
        .finalize();

    LvglArc::new(area, "Arc", 0, 300, 100, 100)
        .set_info("Arc widget")
        .set_value(180)
        .finalize();
}

#[test]
pub fn test_label() {
    let root = display_init().get_root_widget();
    draw_label(root, 100, 100);
    display_loop();
}

#[test]
pub fn test_date() {
    let root = display_init().get_root_widget();
    draw_date(root, 100, 100);
    display_loop();
}

#[test]
pub fn test_icon() {
    let root = display_init().get_root_widget();
    draw_icon(root, 900, 10);
    display_loop();
}

#[test]
pub fn test_led() {
    let root = display_init().get_root_widget();
    draw_led(root, 100, 100);
    display_loop();
}

#[test]
pub fn test_text() {
    let root = display_init().get_root_widget();
    draw_text(root, 100, 100);
    display_loop();
}

#[test]
pub fn test_switch() {
    let root = display_init().get_root_widget();
    draw_switch(root, 100, 100);
    display_loop();
}
#[test]
pub fn test_line() {
    let root = display_init().get_root_widget();
    draw_line(root, 100, 100);
    display_loop();
}
#[test]
pub fn test_button() {
    let root = display_init().get_root_widget();
    draw_button(root, 100, 100);
    display_loop();
}
#[test]
pub fn test_arc() {
    let root = display_init().get_root_widget();
    draw_arc(root, 100, 100);
    display_loop();
}
#[test]
pub fn test_bar() {
    let root = display_init().get_root_widget();
    draw_bar(root, 100, 100);
    display_loop();
}

#[test]
pub fn test_tux() {
    let root = display_init().get_root_widget();
    draw_tux(root, 100, 100);
    display_loop();
}

#[test]
pub fn test_qrcode() {
    let root = display_init().get_root_widget();
    draw_qrcode(root, 100, 100);
    display_loop();
}

#[test]
pub fn test_meter() {
    let root = display_init().get_root_widget();
    draw_meter(root, 100, 100);
    display_loop();
}

#[test]
pub fn test_area() {
    let root = display_init().get_root_widget();
    draw_area(root, 100, 100);
    display_loop();
}

#[test]
pub fn test_pannel() {
    let primary = LvglColor::LIGHT_BLUE();
    let secondary = LvglColor::BLUE_GREY();
    let mut display = display_init();
    display.set_theme(primary, secondary, false, LvglMkFont::std_14());
    let root = display.get_root_widget();
    draw_icon(root, 875, 5);
    draw_led(root, 805, 5);
    draw_date(root, 540, 12);
    draw_switch(root, 805, 50);
    draw_line(root, 400, 70);
    draw_button(root, 450, 200);
    draw_arc(root, 100, 30);
    draw_bar(root, 100, 250);
    draw_meter(root, 800, 350);
    draw_label(root, 240, 400);
    draw_tux(root, 765, 100);
    draw_qrcode(root, 600, 370);
    draw_text(root, 50, 550);
    display_loop();
}
