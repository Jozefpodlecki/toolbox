use iso_viewer::*;


// #[test]
// fn should_build_iso() {
//     let bootloader: [u8; 1024] = [0; 1024];
//     let kernel: [u8; 1024] = [0; 1024];

//     let fat_image = FatBuilder::new()
//         .volume_label("EFI_BOOT")
//         .fat_type(FatType::Fat12)
//         .dir("EFI")
//             .dir("BOOT")
//                 .file("BOOTX64.EFI", bootloader.to_vec())
//             .build()
//         .build();

//     let data = IsoBuilder::new()
//         .volume(|v| {
//             v.name("TEST_VOLUME")
//              .system_id("TEST_SYSTEM")
//         })
//         .add(|fs| {
//             fs.file("kernel", kernel.to_vec())
//               .file("BOOT/EFIBOOT.IMG", fat_image)
//         })
//         .boot(|b| {
//             b.platform(BootPlatform::Both)
//              .image_path("BOOT/EFIBOOT.IMG")
//              .emulation(BootEmulation::NoEmulation)
//              .boot_info_table(true)
//         })
//         .hybrid(true)
//         .sector_size(2048)
//         .build();
// }