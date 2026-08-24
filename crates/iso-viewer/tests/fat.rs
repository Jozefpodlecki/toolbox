use std::io::Cursor;

use hadris_fat::FatVolume;
use iso_viewer::*;

#[test]
fn should_build_fat_12_hadris() -> Result<(), Box<dyn std::error::Error>> {
    let bootloader: [u8; 1024] = [0; 1024];

    let data = {
        let buffer = vec![0u8; 512 * 1024];
        let cursor = Cursor::new(buffer);

        let options = hadris_fat::format::FatFormatOptions::new(512 * 1024)
            .volume_label("EFI_BOOT")
            .fat_type(hadris_fat::format::FatTypeSelection::Fat12);
        let fs = hadris_fat::format::FatVolumeFormatter::format(cursor, options).unwrap();

        {
            let root = fs.root_dir();
            let efi_dir = fs.create_dir(&root, "EFI").unwrap();
            let boot_dir = fs.create_dir(&efi_dir, "BOOT").unwrap();
            
            let file = fs.create_file(&boot_dir, "BOOTX64.EFI").unwrap();
            let mut writer = hadris_fat::FatVolumeWriteExt::write_file(&fs, &file).unwrap();
            writer.write(&bootloader).unwrap();
            writer.finish().unwrap();
        }

        let buffer = fs.into_inner().into_inner();
        println!("{}", buffer.len()); // 524288
        buffer
    };

    let reader = Cursor::new(data);
    let volume: FatVolume<Cursor<Vec<u8>>> = FatVolume::open(reader)?;

    assert_eq!(volume.volume_info().volume_label(), "EFI_BOOT");
    let root_dir = volume.root_dir();

    let dir = root_dir.open_dir("EFI")?;
    let sub_dir = dir.open_dir("BOOT")?;
    let file_entry = sub_dir.find("BOOTX64.EFI")?.unwrap();
    println!("{}", file_entry.len());
    let mut file = sub_dir.open_file("BOOTX64.EFI")?;
    assert_eq!(file.read_to_vec()?, bootloader);

    Ok(())
}

#[test]
fn should_build_fat_32_hadris() -> Result<(), Box<dyn std::error::Error>> {
    let bootloader = vec![0; 32 * 1024 * 1024];

    let data = {
        let cursor = Cursor::new(vec![0; 34 * 1024 * 1024]);

        let options = hadris_fat::format::FatFormatOptions::new(34 * 1024 * 1024)
            .volume_label("EFI_BOOT")
            .fat_type(hadris_fat::format::FatTypeSelection::Fat32);
        let fs = hadris_fat::format::FatVolumeFormatter::format(cursor, options).unwrap();

        {
            let root = fs.root_dir();
            let efi_dir = fs.create_dir(&root, "EFI").unwrap();
            let boot_dir = fs.create_dir(&efi_dir, "BOOT").unwrap();
            
            let file = fs.create_file(&boot_dir, "BOOTX64.EFI").unwrap();
            let mut writer = hadris_fat::FatVolumeWriteExt::write_file(&fs, &file).unwrap();
            writer.write(&bootloader).unwrap();
            writer.finish().unwrap();
        }

        let buffer = fs.into_inner().into_inner();
        buffer
    };

    let reader = Cursor::new(data);
    let volume: FatVolume<Cursor<Vec<u8>>> = FatVolume::open(reader)?;

    assert_eq!(volume.volume_info().volume_label(), "EFI_BOOT");
    let root_dir = volume.root_dir();

     for entry in root_dir.entries() {
        let entry = entry?;
        println!("ITER {}", entry.name());
    }

    let dir = root_dir.open_dir("EFI")?;

     for entry in dir.entries() {
        let entry = entry?;
        println!("SUBDIR {}", entry.name());
    }

    let sub_dir = dir.open_dir("BOOT")?;
    let file_entry = sub_dir.find("BOOTX64.EFI")?.unwrap();
    let mut file = sub_dir.open_file("BOOTX64.EFI")?;
    assert_eq!(file.read_to_vec()?, bootloader);

    Ok(())
}

#[test]
fn should_build_fat_12() -> Result<(), Box<dyn std::error::Error>> {
    let bootloader = vec![0; 1024 * 1024];

     let data = FatBuilder::new("EFI_BOOT")
        .fat_type(FatType::Fat12)
        .dir("TEST", |builder| builder)
        .dir("EFI", |efi| {
            efi.dir("BOOT", |boot| {
                boot.file("BOOTX64.EFI", bootloader.to_vec())
            })
        })
        .build()?;

    let reader = Cursor::new(&data);
    println!("{:?}", FatVolume::open(reader));

    let reader = Cursor::new(data);
    let volume: FatVolume<Cursor<Vec<u8>>> = FatVolume::open(reader)?;

    assert_eq!(volume.volume_info().volume_label(), "EFI_BOOT");
    let root_dir = volume.root_dir();

    for entry in root_dir.entries() {
        let entry = entry?;
        println!("ITER {}", entry.name());
    }
    println!("FINDING ENTRIES");
    
    let dir = root_dir.open_dir("EFI")?;
    let sub_dir = dir.open_dir("BOOT")?;
    let mut file = sub_dir.open_file("BOOTX64.EFI")?;
    assert_eq!(file.read_to_vec()?, bootloader);

    Ok(())
}

#[test]
fn should_build_fat_32() -> Result<(), Box<dyn std::error::Error>> {
     let bootloader = vec![0; 34 * 1024 * 1024];

     let data = FatBuilder::new("EFI_BOOT")
        .fat_type(FatType::Fat32)
        .dir("EFI", |efi| {
            efi.dir("BOOT", |boot| {
                boot.file("BOOTX64.EFI", bootloader.to_vec())
            })
        })
        .build()?;

    let reader = Cursor::new(&data);
    println!("{:?}", FatVolume::open(reader));

    let reader = Cursor::new(data);
    let volume: FatVolume<Cursor<Vec<u8>>> = FatVolume::open(reader)?;

    assert_eq!(volume.volume_info().volume_label(), "EFI_BOOT");
    let root_dir = volume.root_dir();

    for entry in root_dir.entries() {
        let entry = entry?;
        println!("ITER {}", entry.name());
    }
    
    let dir = root_dir.open_dir("EFI")?;
    let sub_dir = dir.open_dir("BOOT")?;
    let mut file = sub_dir.open_file("BOOTX64.EFI")?;
    assert_eq!(file.read_to_vec()?, bootloader);

    Ok(())
}