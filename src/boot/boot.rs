use alloc::string::{String, ToString};
use core::ptr;

use uefi::helpers::system_table;
use uefi::println;
use uefi_graphics2::embedded_graphics::draw_target::DrawTarget;
use uefi_graphics2::embedded_graphics::geometry::Size;
use uefi_graphics2::embedded_graphics::mono_font::ascii::FONT_4X6;
use uefi_graphics2::embedded_graphics::mono_font::MonoTextStyle;
use uefi_graphics2::embedded_graphics::pixelcolor::{Rgb888, RgbColor, WebColors};
use uefi_graphics2::embedded_graphics::prelude::Point;
use uefi_graphics2::embedded_graphics::primitives::{Primitive, PrimitiveStyleBuilder, Rectangle};
use uefi_graphics2::embedded_graphics::text::Text;
use uefi_graphics2::embedded_graphics::Drawable;
use uefi_graphics2::UefiDisplay;

use crate::boot::gop;
use crate::boot::startup::info;
use crate::ui::loader::load_nebula_ui;

pub enum NebulaBootResult {
    Success,
    Error(String),
}

pub fn nebula_boot() -> NebulaBootResult {
    println!("===== NEBULA BOOT STARTED =====");
    let system_table = system_table();
    let boot_services_ref = system_table.boot_services();
    let boot_services = unsafe { ptr::read(boot_services_ref) };

    info::startup_info();

    let mut graphics = match gop::start_gop(&boot_services) {
        Ok(gop) => gop,
        Err(error) => return NebulaBootResult::Error(error.to_string()),
    };

    // tunred off rn cuz it fucks with qemu
    //gop::handle_gop_resolution(&mut gop, &boot_services);

    let mode = graphics.current_mode_info();

    let mut display = UefiDisplay::new(graphics.frame_buffer(), mode);

    let _ = display.fill_solid(
        &Rectangle::with_corners(Point::new(10, 20), Point::new(75, 500)),
        Rgb888::CYAN,
    );

    let style = MonoTextStyle::new(&FONT_4X6, Rgb888::CSS_ALICE_BLUE);
    let text = Text::new("Hello, I am under the water", Point::new(60, 30), style);
    let _ = text.draw(&mut display);

    display.flush();

    println!("===== NEBULA BOOT ENDED =====");

    let rect_x: i32 = 100;
    let rect_y: i32 = 100;

    let mut cx: i32 = 5;
    let mut cy: i32 = 5;
    let mut x: i32 = 200;
    let mut y: i32 = 200;

    loop {
        if x < (mode.resolution().0 as i32 - rect_x) && x > 0 {
            x += cx;
        } else {
            cx = -cx;
            x += cx;
        }

        if y < (mode.resolution().1 as i32 - rect_y) && y > 0 {
            y += cy;
        } else {
            cy = -cy;
            y += cy;
        }

        display.fill_entire(Rgb888::BLACK).unwrap();

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::RED)
            .stroke_width(3)
            .fill_color(Rgb888::GREEN)
            .build();

        let rect = Rectangle::new(
            Point { x, y },
            Size {
                width: rect_x as u32,
                height: rect_y as u32,
            },
        );
        rect.into_styled(style).draw(&mut display).unwrap();

        display.flush();
    }

    let _ = load_nebula_ui(include_bytes!("../assets/ui/test.nui"));

    NebulaBootResult::Success
}
