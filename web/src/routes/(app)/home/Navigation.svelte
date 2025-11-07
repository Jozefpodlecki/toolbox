<script lang="ts">
    import { page } from "$app/state";
    import IconBlocks from "~icons/tabler/blocks";
    import IconRefresh from "~icons/lucide/refresh-cw";

    const onCheck = () => {

    }

</script>

<!-- page.url.pathname === href -->
<nav class="flex flex-col gap-1">
    <a href="/process" class="p-1"><IconBlocks/></a>
    <button type="button" class="p-1 mt-auto" onclick={onCheck}><IconRefresh/></button>
</nav>