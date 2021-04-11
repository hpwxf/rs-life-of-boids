pub fn calculate_relative_brightness(red: u8, green: u8, blue: u8) -> f32 {
    let red_f = (red as f32) / 255.0;
    let green_f = (green as f32) / 255.0;
    let blue_f = (blue as f32) / 255.0;

    f32::sqrt((red_f * red_f) * 0.299 + (green_f * green_f) * 0.587 + (blue_f * blue_f) * 0.114)
}
