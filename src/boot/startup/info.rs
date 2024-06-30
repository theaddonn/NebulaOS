use uefi::helpers::system_table;
use uefi::println;

pub fn startup_info() {
    let system_table = system_table();

    let uefi_revision = system_table.uefi_revision();
    let firmware_revision = system_table.firmware_revision();
    let firmware_vendor = system_table.firmware_vendor();

    println!(
        "UEFI VERSION:      {}.{} ({})",
        uefi_revision.major(),
        uefi_revision.minor(),
        uefi_revision.0
    );
    println!("FIRMWARE VERSION:  {}", firmware_revision);
    println!("FIRMWARE VENDOR:   {}", firmware_vendor);
    println!("BITS:              {}", usize::BITS);
}
