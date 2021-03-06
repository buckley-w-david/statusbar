use std::error::Error;

use async_trait::async_trait;

use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;

pub struct PulseVolumeResource {
    pub average: bool,
}

#[async_trait]
impl resource::Resource for PulseVolumeResource {
    async fn fetch(&self) -> Result<String, Box<dyn Error>> {
        let mut handler = SinkController::create()?;
        let default = handler.get_default_device()?;
        if self.average {
            // Stupid thing appends a "%" by itself when I explicity called out this case
            // in the block trait docs. smh
            let mut vol = format!("{}", default.volume.avg()).trim().to_string();
            vol.truncate(vol.len() - 1);
            Ok(format!("{}", vol))
        } else {
            // Not sure if I should attempt to strip the "%" from this case, it's a little more
            // compliced
            Ok(format!("{}", default.volume))
        }
    }
}
