<script lang="ts">
    import { onMount } from "svelte";
    import ProcessNodeComponent from "./ProcessNode.svelte";
    import { getProcesses } from "$lib/api";
    import IconRefresh from "~icons/lucide/refresh-cw";
    import IconHierarchy from "~icons/tabler/hierarchy-3";
    import IconList from "~icons/tabler/list";
    import type { MouseEventHandler } from "svelte/elements";
    import type { Paged, Process, ProcessNode } from "$lib/types";

    type ProcessResult = 
        | { type: "list"; data: Paged<Process> }
        | { type: "hierarchy"; data: Paged<ProcessNode> };

    interface State {
        result: ProcessResult | null;
        query: string;
        display: "list" | "hierarchy"
    }

    let pageState = $state<State>({
        result: null,
        query: "",
        display: "hierarchy" // "list"
    });

    onMount(() => {
       onLoad();
    })

    async function onLoad() {
        const args = {
            name: pageState.query || null,
            display: pageState.display,
            page: 0,
            pageSize: 10
        }

        const result = await getProcesses(args);
          console.log(result);
        pageState.result = result as any;
    }

    const onQuery = async (value: string) => {
        const args = {
            name: value,
            display: pageState.display,
            page: 0,
            pageSize: 10
        }

        const result = await getProcesses(args);
        console.log(result);
        pageState.result = result as any;
        pageState.query = value;
    }

    const onToggleDisplay: MouseEventHandler<HTMLButtonElement> = async (event) => {
        const display = event.currentTarget.dataset.display! as "list" | "hierarchy";
        pageState.display = display;
        
        await onLoad();
    }

</script>

<div class="flex">
    <input type="text"
        bind:value={() => pageState.query, onQuery}
        class="bg-transparent border-0 border-b w-full"
        placeholder="Search..."/>
    <button type="button" onclick={onLoad} class="p-2 hover:bg-gray-700"><IconRefresh/></button>
    {#if pageState.display === "list" }
        <button type="button" data-display="hierarchy" onclick={onToggleDisplay} class="p-2 hover:bg-gray-700"><IconHierarchy/></button>
    {:else}
        <button type="button" data-display="list" onclick={onToggleDisplay} class="p-2 hover:bg-gray-700"><IconList/></button>
    {/if}
</div>

{#if pageState.result}
    {#if pageState.result.type === "list"}
        {#each pageState.result.data.items as proc (proc.id)}
            <div class="pl-2 border-b border-gray-700 py-1">
                {proc.name} <span class="text-gray-400">#{proc.id}</span>
            </div>
        {/each}
    {:else}
        {#each pageState.result.data.items as node (node.process.id)}
            <ProcessNodeComponent node={node} />
        {/each}
    {/if}
{:else}
    <div class="text-gray-500">Loading...</div>
{/if}