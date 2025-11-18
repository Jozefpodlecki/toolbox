<script lang="ts">
    import { getPrograms } from "$lib/api";
    import { onMount, untrack } from "svelte";
    import Programs from "./Programs.svelte";
    import type { Paged, Program } from "$lib/types";
    import Pagination from "../../../lib/components/Pagination.svelte";
    import IconPackages from "~icons/tabler/packages"; 

    interface State {
        pageSize: number;
        page: number;
        result: Paged<Program>;
    }

    let pageState = $state<State>({
        pageSize: 10,
        page: 0,
        result: {
            items: [],
            page: 0,
            pageSize: 10,
            total: 0
        }
    });

    $effect(() => {
        onLoad()
    })

    async function onLoad() {
        const programs = await getPrograms({
            page: pageState.page,
            pageSize: pageState.pageSize
        });
        pageState.result = programs;
    }

</script>

<div class="flex flex-col h-full items-center">
    <header class="p-1 flex gap-2 items-center">
        Installed Programs <IconPackages/>
    </header>
    <main class="flex-1 w-full overflow-auto">
        {#each pageState.result.items as item}
            <div class="p-1">
                {item.name}
                <!-- {item.path} -->
            </div>
        {/each}
    </main>
    <Pagination
        bind:page={pageState.page}
        bind:pageSize={pageState.pageSize}
        total={pageState.result.total}/>
</div>