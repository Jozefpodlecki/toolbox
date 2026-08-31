use iso_viewer::{GptPartitionInfo, MbrPartitionInfo, PrimaryInfo, SupplementaryInfo};
use yew::prelude::*;
use yew_icons::{Icon, IconData};

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

#[derive(Properties, PartialEq)]
pub struct PrimaryDescriptorInfoRowProps {
    pub descriptor: PrimaryInfo,
}

#[function_component(PrimaryDescriptorInfoRow)]
pub fn primary_descriptor_info_row(props: &PrimaryDescriptorInfoRowProps) -> Html {
    html! {
        <div class="ml-2 border-l-2 border-gray-700 pl-3 space-y-0.5">
            <InfoRow 
                label="Root LBA" 
                value={props.descriptor.root_lba.to_string()} 
            />
            <InfoRow 
                label="Root Size" 
                value={props.descriptor.root_size.as_human_readable()} 
            />
            <InfoRow 
                label="Path Table LPT" 
                value={props.descriptor.path_table.lpt.to_string()} 
            />
            <InfoRow 
                label="Path Table MPT" 
                value={props.descriptor.path_table.mpt.to_string()} 
            />
            <InfoRow 
                label="Path Table Size" 
                value={format!("{} bytes", props.descriptor.path_table.size)} 
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SupplementaryDescriptorInfoRowProps {
    pub descriptor: SupplementaryInfo,
}

#[function_component(SupplementaryDescriptorInfoRow)]
pub fn supplementary_descriptor_info_row(props: &SupplementaryDescriptorInfoRowProps) -> Html {
    html! {
        <div class="ml-2 border-l-2 border-gray-700 pl-3 space-y-0.5">
            <InfoRow 
                label="Root LBA" 
                value={props.descriptor.root_lba.to_string()} 
            />
            <InfoRow 
                label="Root Size" 
                value={props.descriptor.root_size.as_human_readable()} 
            />
            <InfoRow 
                label="Path Table LPT" 
                value={props.descriptor.path_table.lpt.to_string()} 
            />
            <InfoRow 
                label="Path Table MPT" 
                value={props.descriptor.path_table.mpt.to_string()} 
            />
            <InfoRow 
                label="Path Table Size" 
                value={format!("{} bytes", props.descriptor.path_table.size)} 
            />
            <InfoRow 
                label="Joliet Level" 
                value={props.descriptor.joliet_level.map(|l| format!("{:?}", l)).unwrap_or_else(|| "None".to_string())} 
            />
            <InfoRowWithIcon 
                label="Enhanced Volume Descriptor" 
                condition={props.descriptor.is_evd} 
            />
        </div>
    }
}
