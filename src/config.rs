pub const LIGHTNESS: f64 = 0.5; // lightness of the color if for example 0.5 and the screen has
// white the resulting color is 127 127 127
pub const DOWNSCALE_FACTOR: usize = 4; // the downscale factor for the scren
pub const ENABLE_SMOOTHING: bool = true;
pub const SMOOTHING_FACTOR: f32 = 1.0;
pub const COLOR_THRESHOLD: u8 = 10; // threshhold for the leds to refresh
pub const FRAME_DELAY_MS: u64 = 16; // delay betweeen refresh in ms
pub const MODE: &str = "hybrid"; // "avg", "common", "hybrid"
pub const HYBRID_PERCENTAGE: f32 = 0.5; //percentage of the common color
pub const MAX_LIGHTNESS: usize = 255; // The Maximum lightness
pub const MIN_LIGHTNESS: usize = 0; // The Minimum lightness
