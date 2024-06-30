use uefi_graphics2::{UefiDisplay, UefiDisplayError};
use uefi_graphics2::embedded_graphics::draw_target::DrawTarget;
use uefi_graphics2::embedded_graphics::geometry::{Point, Size};
use uefi_graphics2::embedded_graphics::pixelcolor::Rgb888;
use uefi_graphics2::embedded_graphics::primitives::Rectangle;

use crate::ui::component::UiComponent;

struct RectangleUiComponent {
    top_left: Point,
    size: Size,
    color: Rgb888,
}

impl UiComponent for RectangleUiComponent {
    fn draw(&self, draw_target: &mut UefiDisplay) -> Result<(), UefiDisplayError> {
        draw_target.fill_solid(
            &Rectangle {
                top_left: self.top_left,
                size: self.size,
            },
            self.color,
        )
    }
}