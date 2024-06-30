use uefi_services::println;

pub fn startup_info() {
    let system_table = uefi_services::system_table();

    let uefi_revision = system_table.uefi_revision();
    let firmware_revision = system_table.firmware_revision();
    let firmware_vendor = system_table.firmware_vendor();

    println!(
        "UEFI VERSION:      {}.{}",
        uefi_revision.major(),
        uefi_revision.minor()
    );
    println!("FIRMWARE VERSION:  {}", firmware_revision);
    println!("FIRMWARE VENDOR:   {}", firmware_vendor);
}
