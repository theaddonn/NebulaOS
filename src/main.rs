#![no_main]
#![no_std]

extern crate alloc;

use uefi::prelude::*;
use uefi_services::{println, system_table};

use crate::boot::boot::{nebula_boot, NebulaBootResult};

mod boot;
mod nebula;
mod ui;

#[entry]
fn main(_image_handle: Handle, mut boot_system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut boot_system_table).unwrap();

    // Disable the watchdog timer
    boot_system_table
        .boot_services()
        .set_watchdog_timer(0, 0x10000, None)
        .unwrap();

    let boot_result = nebula_boot();

    match boot_result {
        NebulaBootResult::Success => {
            println!("BOOTED SUCCESSFULLY");
        }
        NebulaBootResult::Error(message) => {
            println!("Could not boot: {message}");
            system_table().boot_services().stall(1_000_000);
        }
    }

    println!("MAIN LOOP STARTING");

    system_table().boot_services().stall(1_000_000_000);

    Status::SUCCESS
}
