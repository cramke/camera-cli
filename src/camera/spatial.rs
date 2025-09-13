pub fn spatial_resolution(hfov: f32, pixels: u32, distance: f32) -> f32 {
    let width: f32 = 2.0 * distance * (hfov.to_radians() / 2.0).tan();
    pixels as f32 / width
}

pub fn distance_at_resolution(hfov: f32, pixels: u32, resolution: f32) -> f32 {
    pixels as f32 / (2.0 * resolution * (hfov.to_radians() / 2.0).tan())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_resolution() {
        let angle = 90.0;
        let pixels = 300;
        let distance = 100.0;
        let result = spatial_resolution(angle, pixels, distance);
        // Calculate expected value
        let width = 2.0 * distance * (angle.to_radians() / 2.0).tan();
        let expected = pixels as f32 / width;
        assert!(
            (result - expected).abs() < 1e-6,
            "Expected: {}, Got: {}",
            expected,
            result
        );
    }

    #[test]
    fn test_distance_at_resolution() {
        let hfov = 90.0;
        let pixels = 300;
        let resolution = 25.0;
        let result = distance_at_resolution(hfov, pixels, resolution);
        // Calculate expected value
        let expected = pixels as f32 / (2.0 * resolution * (hfov.to_radians() / 2.0).tan());
        assert!(
            (result - expected).abs() < 1e-6,
            "Expected: {}, Got: {}",
            expected,
            result
        );
    }
}
