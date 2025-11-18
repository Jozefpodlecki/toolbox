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
    <Item href="/"><IconHome/></Item>
    <Item href="/process"><IconBlocks/></Item>
    <Item href="/programs"><IconPackages/></Item>
    <Item href="/memory"><IconDeviceSdCard/></Item>
    <Item href="/driver"><IconDeviceUsb/></Item>
    <Item href="/handles"><IconFile/></Item>
    <Item href="/tcp-table"><IconNetwork/></Item>
    <Item onclick={onCheck} className="mt-auto"><IconRefresh/></Item>
</nav>