quick_error! {
    #[derive(Debug)]
    /// Errors that occur during creation of spherical coordinates;
    pub enum SphericalCreationError {
        NegativeRadius {
            display("Negative radius is invalid.")
        }
        ThetaOutOfBounds {
            display("Theta out of legal range [0, PI]")
        }
        PhiOutOfBounds {
            display("Phi out of legal range [0, 2*PI]")
        }
    }
}

quick_error! {
    /// Errors that occur if invalid arguments are used when changing camera
    /// settings.
    #[derive(Debug)]
    pub enum CameraSettingError {
        InvalidFOV(value: f32) {
            display("Field of view value {} is invalid, allowed values: 0 < FOV < (PI rad or 180 degrees)", value)
        }
    }
}
