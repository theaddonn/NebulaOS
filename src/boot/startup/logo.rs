use alloc::vec::Vec;
use core::{mem, slice};

use uefi::proto::console::gop::{BltPixel, GraphicsOutput};
use uefi::Error;

pub fn startup_logo(
    _alpha: &u8,
    image_data: &Vec<u8>,
    gop: &mut GraphicsOutput,
) -> Result<(), Error> {
    // Reinterpret the data as a slice of BltPixel
    let buffer: &[BltPixel] = unsafe {
        // Ensure that the size of BltPixel matches the size of u8 * 4 (RGBA)
        assert_eq!(mem::size_of::<BltPixel>(), mem::size_of::<u8>() * 4);
        // Cast the data pointer to a pointer of BltPixel and create a slice
        let ptr = image_data.as_ptr() as *const BltPixel;
        slice::from_raw_parts(ptr, image_data.len() / mem::size_of::<BltPixel>())
    };

    Ok(())
}
