use core::fmt::Write;
use alloc::string::String;
use hadris_iso::volume::PrimaryVolumeDescriptor;

use crate::*;

pub struct VolumeDescriptor;

impl VolumeDescriptor {
    pub fn parse<W: Write>(data: &[u8], logger: &mut W) -> IsoResult<(IsoIdentity, Lba, FileSize)> {
        writeln!(logger, "Parsing ISO volume descriptor")?;

        let min_size = (PVD_SECTOR + 1) * ISO_SECTOR_SIZE;
        if data.len() < min_size {
            writeln!(logger, "File too small: need {} bytes, have {}", min_size, data.len())?;
            return Err(IsoError::FileTooSmall { min_size });
        }

        let pvd_offset = PVD_SECTOR * ISO_SECTOR_SIZE;

        // Cast the entire PVD from the data using bytemuck
        let pvd = bytemuck::from_bytes::<PrimaryVolumeDescriptor>(
            &data[pvd_offset..pvd_offset + core::mem::size_of::<PrimaryVolumeDescriptor>()]
        );

        // Validate signature
        let signature = &data[pvd_offset + 1..pvd_offset + 6];
        if signature != b"CD001" {
            return Err(IsoError::InvalidSignature { signature: signature.to_vec() });
        }

        if data[pvd_offset] != 1 {
            return Err(IsoError::MissingPrimaryVolumeDescriptor);
        }

        writeln!(logger, "Valid ISO 9660 signature found")?;

        let identity = IsoIdentity::parse(data, pvd_offset);

        writeln!(logger, "Volume: {:?}, System: {:?}", identity.volume_label, identity.system_id)?;

        // Read root LBA and size from the PVD
        let root_lba = Lba::new(pvd.dir_record.header.extent.read());
        let root_size = FileSize::new(pvd.dir_record.header.data_len.read() as u64);

        writeln!(logger, "Root: LBA {}, size {} bytes", root_lba, root_size)?;

        if root_lba == 0 {
            writeln!(logger, "Root directory not found")?;
            return Err(IsoError::RootDirectoryNotFound);
        }

        Ok((identity, root_lba, root_size))
    }
}