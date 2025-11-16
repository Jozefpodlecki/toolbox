<script lang="ts">
    import { page } from "$app/state";
    import IconHome from "~icons/tabler/home";
    import IconBlocks from "~icons/tabler/blocks";
    import IconPackages from "~icons/tabler/packages"; 
    import IconRefresh from "~icons/lucide/refresh-cw";
    import IconDeviceSdCard from "~icons/tabler/device-sd-card";
    import IconDeviceUsb from "~icons/tabler/device-usb";
    import { checkUpdates } from "$lib/api";
    import Item from "./Item.svelte";

    const onCheck = () => {
        checkUpdates(false)
    }

    $effect(() => {
    console.log(page.url.pathname);
    })
    

</script>

<!-- page.url.pathname === href -->
<nav class="flex flex-col gap-1">
    <Item href="/"><IconHome class="group:text-gray-500"/></Item>
    <Item href="/process"><IconBlocks class="group:text-gray-500"/></Item>
    <Item href="/programs"><IconPackages/></Item>
    <Item href="/memory"><IconDeviceSdCard/></Item>
    <Item href="/driver"><IconDeviceUsb/></Item>
    <Item onclick={onCheck} className="mt-auto"><IconRefresh/></Item>
</nav>