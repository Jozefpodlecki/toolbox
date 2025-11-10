<script lang="ts">
    import { getPrograms } from "$lib/api";
    import { onMount } from "svelte";
    import Programs from "./Programs.svelte";
    import type { Paged, Program } from "$lib/types";

    interface State {
        result: Paged<Program>;
    }

    let pageState = $state<State>({
        result: {
            items: [],
            page: 0,
            total: 0
        }
    });

    onMount(() => {
       onLoad();
    })

    async function onLoad() {
        const programs = await getPrograms({
            page: 0,
            pageSize: 10
        });
        pageState.result = programs;
    }

</script>

<div class="flex">
    {#each pageState.result.items as item}
        <div class="">
            {item.name}
            {item.path}
        </div>
    {/each}
</div>