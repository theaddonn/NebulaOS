use uefi::prelude::BootServices;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::ScopedProtocol;
use uefi::Error;

pub fn start_gop(boot_services: &BootServices) -> Result<ScopedProtocol<GraphicsOutput>, Error> {
    let gop_handle = match boot_services.get_handle_for_protocol::<GraphicsOutput>() {
        Ok(handle) => handle,
        Err(error) => {
            return Err(error);
        }
    };

    let gop: ScopedProtocol<GraphicsOutput> =
        match boot_services.open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
            Ok(gop) => gop,
            Err(error) => {
                return Err(error);
            }
        };

    Ok(gop)
}

pub fn handle_gop_resolution(
    gop: &mut ScopedProtocol<GraphicsOutput>,
    boot_services: &BootServices,
) {
    let mut highest_resolution_mode = None;
    let mut highest_total_resolution = 0;

    for mode in gop.modes(boot_services) {
        let resolution = mode.info().resolution();
        let total_resolution = resolution.0 * resolution.1;

        if total_resolution > highest_total_resolution {
            highest_total_resolution = total_resolution;
            highest_resolution_mode = Some(mode);
        }
    }

    if let Some(mode) = highest_resolution_mode {
        match gop.set_mode(&mode) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
