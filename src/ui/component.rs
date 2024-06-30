use uefi_graphics2::{UefiDisplay, UefiDisplayError};

pub trait UiComponent {
    fn draw(&self, target: &mut UefiDisplay) -> Result<(), UefiDisplayError>;
}
