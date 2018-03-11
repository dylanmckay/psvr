extern crate psvr;

use backend::HeadMountedDevice;

/// A PlayStation VR headset.
pub struct Psvr<'hidapi> {
    /// The underlying PSVR structure.
    psvr: psvr::Psvr<'hidapi>,
}

impl<'a> HeadMountedDevice for Psvr<'a> {
    fn product_name(&self) -> &'static str {
        "PlayStation VR"
    }
}

