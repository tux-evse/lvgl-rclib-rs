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

#define LV_CONF_INCLUDE_SIMPLE 1
#include "lvgl/lvgl.h"

#ifdef USE_FBDEV
#include "lv_drivers/display/fbdev.h"
#include "lv_drivers/indev/evdev.h"
#endif

#if USE_GTK
#include "lv_drivers/gtkdrv/gtkdrv.h"
#endif

#include "../assets/button_left.c"
#include "../assets/button_mid.c"
#include "../assets/button_right.c"
#include "../assets/mouse_cursor.c"

// time of the day C includes
#include <time.h>

const lv_point_t line_points[] = { {5, 5}, {70, 70}, {120, 10}, {180, 60}, {240, 10} };

const short lv_size_contend = LV_COORD_SET_SPEC(2001);

// import some usefull inline macro
lv_color_t lv_color_mk(uint8_t r, uint8_t g, uint8_t b)
{
    return _LV_COLOR_MAKE_TYPE_HELPER LV_COLOR_MAKE(r, g, b);
}

lv_obj_t * lv_scr_action(void)
{
    return lv_disp_get_scr_act(lv_disp_get_default());
}