<script lang="ts">
    import { page } from "$app/state";
    import IconHome from "~icons/tabler/home";
    import IconBlocks from "~icons/tabler/blocks";
    import IconPackages from "~icons/tabler/packages"; 
    import IconRefresh from "~icons/lucide/refresh-cw";
    import IconDeviceSdCard from "~icons/tabler/device-sd-card";
    import IconDeviceUsb from "~icons/tabler/device-usb";
    import IconFile from "~icons/tabler/file";
    import IconNetwork from "~icons/tabler/network";
    import { checkUpdates } from "$lib/api";
    import Item from "./Item.svelte";

    const onCheck = () => {
        checkUpdates(false)
    }

</script>

<nav class="flex flex-col gap-1 [&>[data-selected=true]>svg]:text-gray-500">
    <Item href="/"><IconHome/>Home</Item>
    <Item href="/process"><IconBlocks/> Processes </Item>
    <Item href="/programs"><IconPackages/> Programs</Item>
    <Item href="/memory"><IconDeviceSdCard/> Memory</Item>
    <Item href="/driver"><IconDeviceUsb/> Drivers</Item>
    <Item href="/handles"><IconFile/> Handles</Item>
    <Item href="/net-table"><IconNetwork/> Net Table</Item>
    <Item onclick={onCheck} className="mt-auto"><IconRefresh/></Item>
</nav>