use iso_viewer::{GptPartitionInfo, MbrPartitionInfo};
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::iso::*;

#[function_component(SummaryView)]
pub fn summary_view() -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let state = context.state();

    if let Some(iso) = &state.iso {
        html! {
            <div class="space-y-4 text-white">
                
                <CollapsibleSection title="Volume Information" default_open={true}>
                    <InfoRow label="Volume Label" value={iso.identity.volume_label.as_ref().map(|l| l.as_str().to_string()).unwrap_or_default()} />
                    <InfoRow label="System ID" value={iso.identity.system_id.as_ref().map(|s| s.as_str().to_string()).unwrap_or_default()} />
                    { if let Some(ref app) = iso.identity.application_id {
                        html! { <InfoRow label="Application ID" value={app.clone()} /> }
                    } else { html! {} } }
                    { if let Some(ref pub_id) = iso.identity.publisher_id {
                        html! { <InfoRow label="Publisher ID" value={pub_id.clone()} /> }
                    } else { html! {} } }
                    { if let Some(ref prep) = iso.identity.preparer_id {
                        html! { <InfoRow label="Preparer ID" value={prep.clone()} /> }
                    } else { html! {} } }
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
    } else if let Some(error) = &state.error_message {
        html! {
            <div class="flex items-center justify-center h-full text-red-400">
                <p>{error.as_ref()}</p>
            </div>
        }
    } else {
        html! {
            <div class="flex items-center justify-center h-full text-gray-500">
                <p>{"No ISO loaded"}</p>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleSectionProps {
    pub title: String,
    #[prop_or(false)]
    pub default_open: bool,
    pub children: Children,
}

#[function_component(CollapsibleSection)]
pub fn collapsible_section(props: &CollapsibleSectionProps) -> Html {
    let open = use_state(|| props.default_open);

    let toggle = {
        let open = open.clone();
        Callback::from(move |_| {
            open.set(!*open);
        })
    };

    let icon = match *open {
        true => IconData::LUCIDE_CHEVRON_DOWN,
        false => IconData::LUCIDE_CHEVRON_RIGHT,
    };

    html! {
        <div class="bg-gray-800/30 rounded-lg overflow-hidden border border-gray-700/50">
            <button
                type="button"
                class="w-full flex items-center justify-between px-4 py-2 hover:bg-gray-700/30 transition-colors"
                onclick={toggle}
            >
                <span class="text-sm font-medium text-gray-300">{&props.title}</span>
                <Icon data={icon} class="text-gray-500" width="16px" height="16px" />
            </button>
            if *open {
                <div class="px-4 py-2 space-y-1 border-t border-gray-700/50">
                    { for props.children.iter() }
                </div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct InfoRowProps {
    pub label: String,
    pub value: String,
}

#[function_component(InfoRow)]
pub fn info_row(props: &InfoRowProps) -> Html {
    html! {
        <div class="flex items-center justify-between py-0.5 text-sm">
            <span class="text-gray-400">{&props.label}</span>
            <span class="text-white font-mono text-xs">{&props.value}</span>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct InfoRowWithIconProps {
    pub label: String,
    pub condition: bool,
}

#[function_component(InfoRowWithIcon)]
pub fn info_row_with_icon(props: &InfoRowWithIconProps) -> Html {
    let icon = if props.condition {
        IconData::LUCIDE_CHECK_CIRCLE
    } else {
        IconData::LUCIDE_X_CIRCLE
    };

    let class = if props.condition {
        "text-green-400"
    } else {
        "text-red-400"
    };

    html! {
        <div class="flex items-center justify-between py-0.5 text-sm">
            <span class="text-gray-400">{&props.label}</span>
            <Icon data={icon} class={class} width="18px" height="18px" />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PartitionListProps {
    pub title: String,
    pub partitions: Vec<MbrPartitionInfo>,
}

#[function_component(PartitionList)]
pub fn partition_list(props: &PartitionListProps) -> Html {
    if props.partitions.is_empty() {
        return html! {};
    }

    html! {
        <div class="mt-2">
            <div class="text-xs text-gray-400 mb-1">{&props.title}</div>
            { for props.partitions.iter().map(|p| {
                html! {
                    <InfoRow
                        label={p.partition_type.to_string()}
                        value={format!("{} → {} sectors", p.start_lba, p.sector_count.as_u64())}
                    />
                }
            })}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct GptPartitionListProps {
    pub partitions: Vec<GptPartitionInfo>,
}

#[function_component(GptPartitionList)]
pub fn gpt_partition_list(props: &GptPartitionListProps) -> Html {
    if props.partitions.is_empty() {
        return html! {};
    }

    html! {
        <div class="mt-2">
            <div class="text-xs text-gray-400 mb-1">{"GPT Partitions"}</div>
            { for props.partitions.iter().map(|p| {
                html! {
                    <InfoRow
                        label={p.partition_type.as_str()}
                        value={format!("{} → {} ({})", p.start_lba, p.end_lba, p.size.as_human_readable_short())}
                    />
                }
            })}
        </div>
    }
}