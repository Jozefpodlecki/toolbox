<script lang="ts">
    import { page } from "$app/state";
    import IconHome from "~icons/tabler/home";
    import IconBlocks from "~icons/tabler/blocks";
    import IconPackages from "~icons/tabler/packages"; 
    import IconRefresh from "~icons/lucide/refresh-cw";
    import { checkUpdates } from "$lib/api";

    const onCheck = () => {
        checkUpdates(false)
    }

</script>

<!-- page.url.pathname === href -->
<nav class="flex flex-col gap-1">
    <a href="/" class="p-1"><IconHome/></a>
    <a href="/process" class="p-1"><IconBlocks/></a>
    <a href="/programs" class="p-1"><IconPackages/></a>
    <button type="button" class="p-1 mt-auto" onclick={onCheck}><IconRefresh/></button>
</nav>