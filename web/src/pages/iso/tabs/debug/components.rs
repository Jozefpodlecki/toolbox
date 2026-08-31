use iso_viewer::*;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::iso::{tabs::{debug::*, explorer::find_entries_at_path}, *};

#[derive(Properties, PartialEq)]
pub struct DebugContentProps {
    pub item: DebugItem,
}

#[function_component(DebugContent)]
pub fn debug_content(props: &DebugContentProps) -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let state = context.state();
    let iso = state.iso.as_ref().unwrap();
    let DebugContentProps { item } = props;

    match item {
        DebugItem::PrimaryVolumeDescriptor => {
            html! { <PvdDebug pvd={iso.structures.volume_set.primary.clone()} /> }
        }
        DebugItem::SupplementaryVolumeDescriptor(idx) => {
            if let Some(svd) = iso.structures.volume_set.supplementary.get(*idx) {
                html! { <SvdDebug svd={svd.clone()} idx={*idx} /> }
            } else {
                html! { <div class="text-red-400">{"Supplementary Volume Descriptor not found"}</div> }
            }
        }
        DebugItem::BootCatalog => {
            html! { <BootCatalogDebug catalog={iso.structures.boot_catalog.clone()} /> }
        }
        DebugItem::BootEntry(idx) => {
            if let Some(entry) = iso.structures.boot_catalog.0.get(*idx) {
                html! { <BootEntryDebug entry={entry.clone()} /> }
            } else {
                html! { <div class="text-red-400">{"Boot entry not found"}</div> }
            }
        }
        DebugItem::DirectoryRecord(path) => {
            html! { <DirectoryDebug path={path.clone()} /> }
        }
        DebugItem::Mbr => {
            html! { <MbrDebug partitions={iso.structures.partition_info.mbr_partitions.clone()} /> }
        }
        DebugItem::Gpt => {
            html! { <GptDebug partitions={iso.structures.partition_info.gpt_partitions.clone()} /> }
        }
        DebugItem::PathTable => {
            html! { <PathTableDebug path_table={iso.structures.volume_set.primary.path_table.clone()} /> }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct PvdDebugProps {
    pub pvd: PrimaryInfo,
}

#[function_component(PvdDebug)]
pub fn pvd_debug(props: &PvdDebugProps) -> Html {
    let pvd = &props.pvd;

    html! {
        <div>
            <h3 class="text-sm font-bold text-white mb-3">{"Primary Volume Descriptor"}</h3>
            <div class="space-y-1 font-mono text-xs">
                <DebugRow label="Volume Label" value={DebugValue::from(pvd.identity.volume_label.clone())} />
                <DebugRow label="System ID" value={DebugValue::from(pvd.identity.system_id.clone())} />
                <DebugRow label="Volume Set ID" value={DebugValue::from(pvd.identity.volume_set_id.clone())} />
                <DebugRow label="Publisher ID" value={DebugValue::from(pvd.identity.publisher_id.clone())} />
                <DebugRow label="Preparer ID" value={DebugValue::from(pvd.identity.preparer_id.clone())} />
                <DebugRow label="Application ID" value={DebugValue::from(pvd.identity.application_id.clone())} />
                <DebugRow label="Creation Date" value={DebugValue::Date(pvd.identity.creation_date.clone().unwrap_or_default())} />
                <DebugRow label="Modification Date" value={DebugValue::Date(pvd.identity.modification_date.clone().unwrap_or_default())} />
                <DebugRow label="Expiration Date" value={DebugValue::Date(pvd.identity.expiration_date.clone().unwrap_or_default())} />
                <DebugRow label="Effective Date" value={DebugValue::Date(pvd.identity.effective_date.clone().unwrap_or_default())} />
                <DebugRow label="Root LBA" value={DebugValue::Number(pvd.root_lba.as_u32() as u64)} />
                <DebugRow label="Root Size" value={DebugValue::Size(pvd.root_size.as_u64())} />
                <DebugRow label="L-Path Table LBA" value={DebugValue::Number(pvd.path_table.lpt.as_u32() as u64)} />
                <DebugRow label="M-Path Table LBA" value={DebugValue::Number(pvd.path_table.mpt.as_u32() as u64)} />
                <DebugRow label="Path Table Size" value={DebugValue::Size(pvd.path_table.size)} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SvdDebugProps {
    pub svd: SupplementaryInfo,
    pub idx: usize,
}

#[function_component(SvdDebug)]
pub fn svd_debug(props: &SvdDebugProps) -> Html {
    let svd = &props.svd;

    html! {
        <div>
            <h3 class="text-sm font-bold text-white mb-3">{"Supplementary Volume Descriptor #"}{props.idx}</h3>
            <div class="space-y-1 font-mono text-xs">
                <DebugRow label="Volume Label" value={DebugValue::from(svd.identity.volume_label.clone())} />
                <DebugRow label="System ID" value={DebugValue::from(svd.identity.system_id.clone())} />
                <DebugRow label="Volume Set ID" value={DebugValue::from(svd.identity.volume_set_id.clone())} />
                <DebugRow label="Publisher ID" value={DebugValue::from(svd.identity.publisher_id.clone())} />
                <DebugRow label="Preparer ID" value={DebugValue::from(svd.identity.preparer_id.clone())} />
                <DebugRow label="Application ID" value={DebugValue::from(svd.identity.application_id.clone())} />
                <DebugRow label="Root LBA" value={DebugValue::Number(svd.root_lba.as_u32() as u64)} />
                <DebugRow label="Root Size" value={DebugValue::Size(svd.root_size.as_u64())} />
                <DebugRow label="L-Path Table LBA" value={DebugValue::Number(svd.path_table.lpt.as_u32() as u64)} />
                <DebugRow label="M-Path Table LBA" value={DebugValue::Number(svd.path_table.mpt.as_u32() as u64)} />
                <DebugRow label="Path Table Size" value={DebugValue::Size(svd.path_table.size)} />
                <DebugRow label="Joliet Level" value={DebugValue::String(format!("{:?}", svd.joliet_level))} />
                <DebugRow label="Enhanced Volume Descriptor" value={DebugValue::Boolean(svd.is_evd)} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BootCatalogDebugProps {
    pub catalog: BootCatalogInfo,
}

#[function_component(BootCatalogDebug)]
pub fn boot_catalog_debug(props: &BootCatalogDebugProps) -> Html {
    html! {
        <div>
            <h3 class="text-sm font-bold text-white mb-3">{"Boot Catalog"}</h3>
            <div class="space-y-1 font-mono text-xs">
                <DebugRow label="Entries" value={DebugValue::Number(props.catalog.len() as u64)} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BootEntryDebugProps {
    pub entry: BootEntryInfo,
}

#[function_component(BootEntryDebug)]
pub fn boot_entry_debug(props: &BootEntryDebugProps) -> Html {
    let entry = &props.entry;

    html! {
        <div>
            <h3 class="text-sm font-bold text-white mb-3">{"Boot Entry"}</h3>
            <div class="space-y-1 font-mono text-xs">
                <DebugRow label="Platform" value={DebugValue::String(entry.platform.as_str().to_string())} />
                <DebugRow label="Bootable" value={DebugValue::Boolean(entry.bootable)} />
                <DebugRow label="LBA" value={DebugValue::Number(entry.lba.as_u32() as u64)} />
                <DebugRow label="Sectors" value={DebugValue::Number(entry.sectors.as_u32() as u64)} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DirectoryDebugProps {
    pub path: String,
}

#[function_component(DirectoryDebug)]
pub fn directory_debug(props: &DirectoryDebugProps) -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let state = context.state();
    let iso = state.iso.as_ref().unwrap();
    let entries = find_entries_at_path(&iso.structures.root_entries.0, &props.path);

    html! {
        <div>
            <h3 class="text-sm font-bold text-white mb-3">{"Directory: "}{&props.path}</h3>
            <div class="overflow-auto">
                <table class="w-full text-xs font-mono">
                    <thead>
                        <tr class="text-gray-500 border-b border-gray-700">
                            <th class="text-left py-1 px-2">{"Name"}</th>
                            <th class="text-left py-1 px-2">{"Type"}</th>
                            <th class="text-right py-1 px-2">{"LBA"}</th>
                            <th class="text-right py-1 px-2">{"Size"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for entries.iter().map(|entry| {
                            html! {
                                <tr class="border-b border-gray-700/50 hover:bg-gray-700/30">
                                    <td class="py-1 px-2 text-white">{entry.name.as_str()}</td>
                                    <td class="py-1 px-2 text-gray-400">
                                        {if entry.is_directory { "Directory" } else { "File" }}
                                    </td>
                                    <td class="py-1 px-2 text-right text-gray-400">{entry.lba.to_string()}</td>
                                    <td class="py-1 px-2 text-right text-gray-400">{FormattedSize::from(entry.size.as_u64()).to_string()}</td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MbrDebugProps {
    pub partitions: Option<Vec<MbrPartitionInfo>>,
}

#[function_component(MbrDebug)]
pub fn mbr_debug(props: &MbrDebugProps) -> Html {
    let partitions = props.partitions.clone().unwrap_or_default();

    html! {
        <div>
            <h3 class="text-sm font-bold text-white mb-3">{"MBR Partitions"}</h3>
            { if partitions.is_empty() {
                html! { <div class="text-gray-500 text-xs">{"No MBR partitions found"}</div> }
            } else {
                html! {
                    <div class="space-y-2 font-mono text-xs">
                        { for partitions.iter().enumerate().map(|(i, p)| {
                            html! {
                                <div class="border border-gray-700 rounded p-3">
                                    <h4 class="text-gray-400 font-bold mb-1">{"Partition #"}{i}</h4>
                                    <DebugRow label="Type" value={DebugValue::String(p.partition_type.to_string())} />
                                    <DebugRow label="Start LBA" value={DebugValue::Number(p.start_lba.as_u64())} />
                                    <DebugRow label="Sector Count" value={DebugValue::Number(p.sector_count.as_u64())} />
                                    <DebugRow label="Size" value={DebugValue::Size(p.sector_count.as_u64() * 512)} />
                                    <DebugRow label="Bootable" value={DebugValue::Boolean(p.bootable)} />
                                </div>
                            }
                        })}
                    </div>
                }
            }}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct GptDebugProps {
    pub partitions: Option<Vec<GptPartitionInfo>>,
}

#[function_component(GptDebug)]
pub fn gpt_debug(props: &GptDebugProps) -> Html {
    let partitions = props.partitions.clone().unwrap_or_default();

    html! {
        <div>
            <h3 class="text-sm font-bold text-white mb-3">{"GPT Partitions"}</h3>
            { if partitions.is_empty() {
                html! { <div class="text-gray-500 text-xs">{"No GPT partitions found"}</div> }
            } else {
                html! {
                    <div class="space-y-2 font-mono text-xs">
                        { for partitions.iter().enumerate().map(|(i, p)| {
                            let size = (p.end_lba - p.start_lba + 1) * 512;
                            html! {
                                <div class="border border-gray-700 rounded p-3">
                                    <h4 class="text-gray-400 font-bold mb-1">{"Partition #"}{i}</h4>
                                    <DebugRow label="Type" value={DebugValue::String(p.partition_type.as_str().to_string())} />
                                    <DebugRow label="Start LBA" value={DebugValue::Number(p.start_lba.as_u64())} />
                                    <DebugRow label="End LBA" value={DebugValue::Number(p.end_lba.as_u64())} />
                                    <DebugRow label="Size" value={DebugValue::Size(size as u64)} />
                                </div>
                            }
                        })}
                    </div>
                }
            }}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PathTableDebugProps {
    pub path_table: PathTableInfo,
}

#[function_component(PathTableDebug)]
pub fn path_table_debug(props: &PathTableDebugProps) -> Html {
    let pt = &props.path_table;

    html! {
        <div>
            <h3 class="text-sm font-bold text-white mb-3">{"Path Table"}</h3>
            <div class="space-y-1 font-mono text-xs">
                <DebugRow label="L-Path Table LBA" value={DebugValue::Number(pt.lpt.as_u32() as u64)} />
                <DebugRow label="M-Path Table LBA" value={DebugValue::Number(pt.mpt.as_u32() as u64)} />
                <DebugRow label="Size" value={DebugValue::Size(pt.size)} />
            </div>
        </div>
    }
}