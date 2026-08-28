use std::io::Cursor;

use hadris_fat::FatVolume;
use iso_viewer::*;

fn build_fat(
    fat_type: FatType,
    bootloader: &[u8],
) -> Result<Vec<u8>, FatError> {
    FatBuilder::new("EFI_BOOT")
        .fat_type(fat_type)
        .dir("EFI", |efi| {
            efi.dir("BOOT", |boot| {
                boot.file("BOOTX64.EFI", bootloader.to_vec())
            })
        })
        .build()
}

#[test]
fn should_build_fat_12() -> Result<(), Box<dyn std::error::Error>> {
    let bootloader = vec![0; 1024 * 1024];

    let data = build_fat(FatType::Fat12, &bootloader)?;

    let reader = Cursor::new(data);
    let volume: FatVolume<Cursor<Vec<u8>>> =
        FatVolume::open(reader)?;

    assert_eq!(
        volume.volume_info().volume_label(),
        "EFI_BOOT"
    );

    let root_dir = volume.root_dir();
    let dir = root_dir.open_dir("EFI")?;
    let sub_dir = dir.open_dir("BOOT")?;
    let mut file = sub_dir.open_file("BOOTX64.EFI")?;

    assert_eq!(file.read_to_vec()?, bootloader);

    Ok(())
}

#[test]
fn should_build_fat_16() -> Result<(), Box<dyn std::error::Error>> {
    let bootloader = vec![0; 1024 * 1024];

    let data = build_fat(FatType::Fat16, &bootloader)?;

    let reader = Cursor::new(data);
    let volume: FatVolume<Cursor<Vec<u8>>> =
        FatVolume::open(reader)?;

    assert_eq!(
        volume.volume_info().volume_label(),
        "EFI_BOOT"
    );

    let root_dir = volume.root_dir();
    let dir = root_dir.open_dir("EFI")?;
    let sub_dir = dir.open_dir("BOOT")?;
    let mut file = sub_dir.open_file("BOOTX64.EFI")?;
    println!("file.size() {}", file.size());
    println!("volume_info = {:?}", volume.volume_info());
    let actual = file.read_to_vec()?;


    assert_eq!(actual.len(), bootloader.len());

    for (i, (actual_chunk, expected_chunk)) in actual
        .chunks(4096)
        .zip(bootloader.chunks(4096))
        .enumerate()
    {
        assert_eq!(
            actual_chunk,
            expected_chunk,
            "mismatch in chunk {} (offset 0x{:X})",
            i,
            i * 4096,
        );
    }

    Ok(())
}

#[test]
fn should_build_fat_32() -> Result<(), Box<dyn std::error::Error>> {
    let bootloader = vec![0; 34 * 1024 * 1024];

    let data = build_fat(FatType::Fat32, &bootloader)?;

    let reader = Cursor::new(data);
    let volume: FatVolume<Cursor<Vec<u8>>> =
        FatVolume::open(reader)?;

    assert_eq!(
        volume.volume_info().volume_label(),
        "EFI_BOOT"
    );

    let root_dir = volume.root_dir();
    let dir = root_dir.open_dir("EFI")?;
    let sub_dir = dir.open_dir("BOOT")?;
    let mut file = sub_dir.open_file("BOOTX64.EFI")?;

    assert_eq!(file.read_to_vec()?, bootloader);

    Ok(())
}