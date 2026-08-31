use core::fmt::Write;
use alloc::{string::String, vec::Vec};
use hadris_iso::{joliet::JolietLevel, types::Endian, volume::{PrimaryVolumeDescriptor, SupplementaryVolumeDescriptor}};

use crate::{constants::*, *};

pub struct VolumeDescriptor;

impl VolumeDescriptor {
    pub fn parse<W: Write>(data: &[u8], logger: &mut W) -> IsoResult<VolumeSet> {
        writeln!(logger, "Parsing ISO volume descriptor")?;

        let min_size = (PVD_SECTOR + 1) * ISO_SECTOR_SIZE;
        if data.len() < min_size {
            writeln!(logger, "File too small: need {} bytes, have {}", min_size, data.len())?;
            return Err(IsoError::FileTooSmall { min_size });
        }

        let primary = Self::parse_primary(data, logger)?;
        let supplementary = Self::parse_supplementary(data, logger)?;

        Ok(VolumeSet {
            primary,
            supplementary,
        })
    }

    fn parse_primary<W: Write>(data: &[u8], logger: &mut W) -> IsoResult<PrimaryInfo> {
        let pvd_offset = PVD_SECTOR * ISO_SECTOR_SIZE;

        let pvd = bytemuck::from_bytes::<PrimaryVolumeDescriptor>(
            &data[pvd_offset..pvd_offset + core::mem::size_of::<PrimaryVolumeDescriptor>()]
        );

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

        let root_lba = Lba::new(pvd.dir_record.header.extent.read());
        let root_size = FileSize::new(pvd.dir_record.header.data_len.read() as u64);

        writeln!(logger, "Root: LBA {}, size {} bytes", root_lba, root_size)?;

        if root_lba == 0 {
            writeln!(logger, "Root directory not found")?;
            return Err(IsoError::RootDirectoryNotFound);
        }

        let path_table = PathTableInfo {
            lpt: Lba::new(pvd.type_l_path_table.get()),
            mpt: Lba::new(pvd.type_m_path_table.get()),
            size: pvd.path_table_size.read() as u64,
        };

        writeln!(logger, "Path Table: LPT LBA {}, MPT LBA {}, size {} bytes", 
            path_table.lpt, path_table.mpt, path_table.size)?;

        Ok(PrimaryInfo {
            identity,
            root_lba,
            root_size,
            path_table,
        })
    }

    fn parse_supplementary<W: Write>(data: &[u8], logger: &mut W) -> IsoResult<Vec<SupplementaryInfo>> {
        let mut descriptors = Vec::new();
        let mut sector = PVD_SECTOR + 1;

        while sector * ISO_SECTOR_SIZE + 7 < data.len() {
            let offset = sector * ISO_SECTOR_SIZE;
            let header_type = data[offset];
            let signature = &data[offset + 1..offset + 6];

            if signature != b"CD001" {
                break;
            }

            if header_type == 2 {
                let svd = bytemuck::from_bytes::<SupplementaryVolumeDescriptor>(
                    &data[offset..offset + core::mem::size_of::<SupplementaryVolumeDescriptor>()]
                );

                let version = svd.header.version;
                let is_evd = version == 2;
                let joliet_level = JolietLevel::from_escape_sequence(&svd.escape_sequences);

                let identity = IsoIdentity::parse(data, offset);

                let root_lba = Lba::new(svd.dir_record.header.extent.read());
                let root_size = FileSize::new(svd.dir_record.header.data_len.read() as u64);

                let path_table = PathTableInfo {
                    lpt: Lba::new(svd.type_l_path_table.get()),
                    mpt: Lba::new(svd.type_m_path_table.get()),
                    size: svd.path_table_size.read() as u64,
                };

                writeln!(logger, "Supplementary Volume Descriptor found (version {}, EVD: {}, Joliet: {:?})", 
                    version, is_evd, joliet_level)?;

                descriptors.push(SupplementaryInfo {
                    identity,
                    root_lba,
                    root_size,
                    path_table,
                    is_evd,
                    joliet_level,
                });
            }

            if header_type == 0xFF {
                break;
            }

            sector += 1;
        }

        Ok(descriptors)
    }

}