use std::error::Error;

use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;

pub struct VolumeBlock {
    pub average: bool
}

impl block::Block for VolumeBlock {
    fn perform(&self) -> Result<String, Box<dyn Error>> {
        let mut handler = SinkController::create()?;
        let default = handler.get_default_device()?;
        if self.average {
            Ok(format!("{}", default.volume.avg()).trim().to_string())
        } else {
            Ok(format!("{}", default.volume))
        }

    }
}
