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
}
