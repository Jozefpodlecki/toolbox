<script lang="ts">
    import { page } from "$app/state";
    import IconHome from "~icons/tabler/home";
    import IconBlocks from "~icons/tabler/blocks";
    import IconPackages from "~icons/tabler/packages"; 
    import IconRefresh from "~icons/lucide/refresh-cw";
    import Tooltip from "./Tooltip.svelte";

    const onCheck = () => {

    }

</script>

<!-- page.url.pathname === href -->
<nav class="flex flex-col gap-1">
    <Tooltip text="Dashboard"><a href="/" class="p-1"><IconHome/></a></Tooltip>
    <Tooltip text="Processes"><a href="/process" class="p-1"><IconBlocks/></a></Tooltip>
    <Tooltip text="Programs"><a href="/programs" class="p-1"><IconPackages/></a></Tooltip>
    <button type="button" class="p-1 mt-auto" onclick={onCheck}><IconRefresh/></button>
</nav>