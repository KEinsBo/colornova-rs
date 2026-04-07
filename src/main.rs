mod capture;
mod color;
mod config;
mod led;
mod utils;

use crate::capture::ScreenCapture;
use crate::color::{color_changed, smooth_color};
use crate::config::*;
use crate::led::set_all_leds_client;
use crate::utils::downscale;
use openrgb2::Color as OpenRgbColor;
use openrgb2::OpenRgbClient;
use std::time::Instant;
use tokio::time::sleep;
#[tokio::main]
async fn main() -> openrgb2::OpenRgbResult<()> {
    run().await
}

pub async fn run() -> openrgb2::OpenRgbResult<()> {
    let mut capture = ScreenCapture::new();
    let (w, h) = capture.dimensions();
    let mut prev_color = OpenRgbColor { r: 0, g: 0, b: 0 };
    let mut last_update = Instant::now();
    let client = OpenRgbClient::connect().await?;
    loop {
        let buffer = capture.get_frame();
        if buffer.is_empty() {
            continue;
        }

        let buf = downscale(buffer, w, h, DOWNSCALE_FACTOR);

        let color: crate::color::Color;
        let stride = w / DOWNSCALE_FACTOR * 4;
        if MODE == "avg" {
            color = crate::color::avg_color(&buf, stride, h / DOWNSCALE_FACTOR);
        } else if MODE == "common" {
            color = crate::color::most_frequent_color(&buf, stride, h / DOWNSCALE_FACTOR);
        } else {
            let color_avg = crate::color::avg_color(&buf, stride, h / DOWNSCALE_FACTOR);
            let color_common =
                crate::color::most_frequent_color(&buf, stride, h / DOWNSCALE_FACTOR);
            let avg_factor = 1.0 - HYBRID_PERCENTAGE;
            color = crate::color::Color {
                r: (color_avg.r as f32 * avg_factor + color_common.r as f32 * HYBRID_PERCENTAGE)
                    as u8,
                g: (color_avg.g as f32 * avg_factor + color_common.g as f32 * HYBRID_PERCENTAGE)
                    as u8,
                b: (color_avg.b as f32 * avg_factor + color_common.b as f32 * HYBRID_PERCENTAGE)
                    as u8,
            }
        }
        let calc_color = OpenRgbColor {
            r: (color.r as f64 * LIGHTNESS)
                .min(MAX_LIGHTNESS as f64)
                .max(MIN_LIGHTNESS as f64) as u8,
            g: (color.g as f64 * LIGHTNESS)
                .min(MAX_LIGHTNESS as f64)
                .max(MIN_LIGHTNESS as f64) as u8,
            b: (color.b as f64 * LIGHTNESS)
                .min(MAX_LIGHTNESS as f64)
                .max(MIN_LIGHTNESS as f64) as u8,
        };

        let led_color = if ENABLE_SMOOTHING {
            let delta_time = Instant::now().duration_since(last_update).as_secs_f32();
            last_update = Instant::now();
            smooth_color(
                &prev_color,
                &calc_color,
                (delta_time * SMOOTHING_FACTOR).min(1.0),
            )
        } else {
            calc_color
        };

        if ENABLE_SMOOTHING || color_changed(&led_color, &prev_color, COLOR_THRESHOLD) {
            set_all_leds_client(&client, 0, led_color).await?;
            set_all_leds_client(&client, 1, led_color).await?;
        }

        prev_color = led_color;
        sleep(std::time::Duration::from_millis(FRAME_DELAY_MS)).await;
    }
}
