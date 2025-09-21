pub mod spatial;

pub fn detection_distance(hfov: f32, pixels: u32) -> f32 {
    const DETECTION_PPM: f32 = 25.0; // pixels/meter
    spatial::distance_at_resolution(hfov, pixels, DETECTION_PPM)
}

pub fn observation_distance(hfov: f32, pixels: u32) -> f32 {
    const OBSERVATION_PPM: f32 = 62.5; // pixels/meter
    spatial::distance_at_resolution(hfov, pixels, OBSERVATION_PPM)
}

pub fn recognition_distance(hfov: f32, pixels: u32) -> f32 {
    const RECOGNITION_PPM: f32 = 125.0; // pixels/meter
    spatial::distance_at_resolution(hfov, pixels, RECOGNITION_PPM)
}

pub fn identification_distance(hfov: f32, pixels: u32) -> f32 {
    const IDENTIFICATION_PPM: f32 = 250.0; // pixels/meter
    spatial::distance_at_resolution(hfov, pixels, IDENTIFICATION_PPM)
}

pub fn focal_length_to_hfov(
    focal_length_mm: f32,
    working_distance_mm: f32,
    sensor_width_mm: f32,
) -> f32 {
    let fov_spatial: f32 = sensor_width_mm * working_distance_mm / focal_length_mm;
    let fov_angle: f32 = 2.0 * (fov_spatial / (2.0 * working_distance_mm)).atan();
    fov_angle.to_degrees()
}

pub fn focal_length_to_opening_angle(focal_length_mm: f32, sensor_width_mm: f32) -> f32 {
    let fov_angle: f32 = 2.0 * (sensor_width_mm / (2.0 * focal_length_mm)).atan();
    fov_angle.to_degrees()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection_distance() {
        let angle = 90.0;
        let pixels = 300;
        let result = detection_distance(angle, pixels);
        // Calculate expected value
        let expected = 6.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Expected: {}, Got: {}",
            expected,
            result
        );
    }

    #[test]
    fn test_observation_distance() {
        let angle = 90.0;
        let pixels = 300;
        let result = observation_distance(angle, pixels);
        // Calculate expected value
        let expected = 2.4;
        assert!(
            (result - expected).abs() < 1e-6,
            "Expected: {}, Got: {}",
            expected,
            result
        );
    }

    #[test]
    fn test_recognition_distance() {
        let angle = 90.0;
        let pixels = 300;
        let result = recognition_distance(angle, pixels);
        // Calculate expected value
        let expected = 1.2;
        assert!(
            (result - expected).abs() < 1e-6,
            "Expected: {}, Got: {}",
            expected,
            result
        );
    }

    #[test]
    fn test_identification_distance() {
        let angle = 90.0;
        let pixels = 300;
        let result = identification_distance(angle, pixels);
        // Calculate expected value
        let expected = 0.6;
        assert!(
            (result - expected).abs() < 1e-6,
            "Expected: {}, Got: {}",
            expected,
            result
        );
    }

    #[test]
    fn test_focal_length_conversion() {
        let focal_length = 8.0; // mm
        let working_distance = 200.0; // mm
        let sensor_width = 2.0; // mm
        let result = focal_length_to_hfov(focal_length, working_distance, sensor_width);
        // Calculate expected value
        let expected = 14.25; // degrees
        assert!(
            (result - expected).abs() < 1e-4,
            "Expected: {}, Got: {}",
            expected,
            result
        );
    }

    #[test]
    fn test_focal_length_to_opening_angle() {
        let focal_length: f32 = 50.0; // mm
        let sensor_width: f32 = 36.0; // mm
        let result: f32 = focal_length_to_opening_angle(focal_length, sensor_width);
        let expected: f32 = 39.59776; // degrees
        assert!(
            (result - expected).abs() < 1e-4,
            "Expected: {}, Got: {}",
            expected,
            result
        );
    }
}
