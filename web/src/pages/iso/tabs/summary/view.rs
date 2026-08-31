use iso_viewer::{GptPartitionInfo, MbrPartitionInfo};
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::iso::{tabs::summary::*, *};

#[function_component(SummaryView)]
pub fn summary_view() -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let state = context.state();
    let iso: &iso_viewer::IsoInfo = state.iso.as_ref().unwrap();

    html! {
        <div class="space-y-4 text-white">
            
             <CollapsibleSection title="Volume Information" default_open={true}>
                <InfoRow 
                    label="Volume Label" 
                    value={iso.structures.volume_set.primary.identity.volume_label.as_ref().map(|l| l.as_str().to_string()).unwrap_or_default()} 
                />
                <InfoRow 
                    label="System ID" 
                    value={iso.structures.volume_set.primary.identity.system_id.as_ref().map(|s| s.as_str().to_string()).unwrap_or_default()} 
                />
                { if let Some(ref app) = iso.structures.volume_set.primary.identity.application_id {
                    html! { <InfoRow label="Application ID" value={app.clone()} /> }
                } else { html! {} } }
                { if let Some(ref pub_id) = iso.structures.volume_set.primary.identity.publisher_id {
                    html! { <InfoRow label="Publisher ID" value={pub_id.clone()} /> }
                } else { html! {} } }
                { if let Some(ref prep) = iso.structures.volume_set.primary.identity.preparer_id {
                    html! { <InfoRow label="Preparer ID" value={prep.clone()} /> }
                } else { html! {} } }
                { if let Some(ref set_id) = iso.structures.volume_set.primary.identity.volume_set_id {
                    html! { <InfoRow label="Volume Set ID" value={set_id.clone()} /> }
                } else { html! {} } }
            </CollapsibleSection>

            <CollapsibleSection title="Volume Descriptors">
                <div class="space-y-2">
                    <div class="text-xs text-gray-400">{"Primary Volume Descriptor"}</div>
                    <PrimaryDescriptorInfoRow descriptor={iso.structures.volume_set.primary.clone()} />
                    
                    { if !iso.structures.volume_set.supplementary.is_empty() {
                        html! {
                            <>
                                <div class="text-xs text-gray-400 mt-2">{"Supplementary Volume Descriptors"}</div>
                                { for iso.structures.volume_set.supplementary.iter().cloned().map(|desc| {
                                    html! { <SupplementaryDescriptorInfoRow descriptor={desc} /> }
                                })}
                            </>
                        }
                    } else {
                        html! { <div class="text-xs text-gray-500 ml-2">{"No supplementary descriptors"}</div> }
                    }}
                </div>
            </CollapsibleSection>

            <CollapsibleSection title="Size Information">
                <InfoRow label="Total Size" value={iso.stats.total_size.as_human_readable()} />
                <InfoRow label="Sector Size" value={format!("{} bytes", iso.stats.sector_size)} />
                <InfoRow label="Total Sectors" value={iso.stats.total_sectors.to_string()} />
            </CollapsibleSection>

            <CollapsibleSection title="File System">
                <InfoRow label="Total Files" value={iso.stats.file_count.to_string()} />
                <InfoRow label="Total Directories" value={iso.stats.directory_count.to_string()} />
                <InfoRow label="Max Depth" value={iso.stats.max_depth.to_string()} />
            </CollapsibleSection>

            <CollapsibleSection title="Partition Table">
                <InfoRowWithIcon label="MBR" condition={iso.structures.partition_info.has_mbr} />
                <InfoRowWithIcon label="GPT" condition={iso.structures.partition_info.has_gpt} />
                <InfoRowWithIcon label="Hybrid" condition={iso.structures.partition_info.is_hybrid} />

                <PartitionList
                    title="MBR Partitions"
                    partitions={iso.structures.partition_info.mbr_partitions.clone().unwrap_or_default()}
                />

                <GptPartitionList
                    partitions={iso.structures.partition_info.gpt_partitions.clone().unwrap_or_default()}
                />
            </CollapsibleSection>

            <CollapsibleSection title="Boot Information">
                <InfoRowWithIcon label="Boot Catalog" condition={!iso.structures.boot_catalog.is_empty()} />
                <InfoRow label="Boot Entries" value={iso.structures.boot_catalog.len().to_string()} />

                { for iso.structures.boot_catalog.iter().map(|entry| {
                    html! {
                        <div class="ml-4 mt-1 border-l-2 border-gray-700 pl-4">
                            <InfoRow label="Platform" value={entry.platform.as_str().to_string()} />
                            <InfoRowWithIcon label="Bootable" condition={entry.bootable} />
                            <InfoRow label="LBA" value={entry.lba.to_string()} />
                            <InfoRow label="Sectors" value={entry.sectors.to_string()} />
                        </div>
                    }
                })}
            </CollapsibleSection>

            { if !iso.structures.metadata.is_empty() {
                html! {
                    <CollapsibleSection title="Additional Metadata">
                        { for iso.structures.metadata.iter().map(|(key, value)| {
                            html! { <InfoRow label={key.as_str().to_string()} value={value.clone()} /> }
                        })}
                    </CollapsibleSection>
                }
            } else { html! {} } }
        </div>
    }
}
