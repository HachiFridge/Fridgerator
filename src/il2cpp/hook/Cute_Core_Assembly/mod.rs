use crate::core::{game::Region, Fridgerator};

mod SafetyNet;
mod Device;

pub fn init() {
    get_assembly_image_or_return!(image, "Cute.Core.Assembly.dll");

    // Taiwan version doesn't have SafetyNet implemented
    if Fridgerator::instance().game.region != Region::Taiwan {
        SafetyNet::init(image);
    }
    Device::init(image);
}