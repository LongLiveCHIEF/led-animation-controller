#![no_std]

mod LightingController {
    pub struct AnimationController {
        numLeds: u32,
    };

    impl AnimationController {
        pub fn new(numLeds: u32) -> Self {
            Self {
                numLeds,
            }
        }
    }

}
