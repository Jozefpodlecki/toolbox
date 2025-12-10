<script lang="ts">
    import { onMount } from "svelte";
    import { getNetTable } from "$lib/api";
    import IconNetwork from "~icons/tabler/network";
    import type { GetNetTableArgs, Paged, NetTableEntry } from "$lib/types";
    import Pagination from "$lib/components/Pagination.svelte";

    interface LoadedState extends GetNetTableArgs {
        result: Paged<NetTableEntry>;
    }

    type State =
    | ({ isLoading: true } & GetNetTableArgs)
    | ({ isLoading: false } & LoadedState);

    let pageState = $state<State>({
        localIpAddr: null,
        localPort: null,
        processName: null,
        remotePort: null,
        remoteIpAddr: null,
        protocols: ["tcp", "udp"],
        pageSize: 10,
        page: 0,
        isLoading: true
    });

    onMount(() => {
       onLoad();
    })

    async function onLoad() {
        const args: GetNetTableArgs = {
            ...pageState
        };

       const result = await getNetTable(args);

        pageState = {
            ...pageState,
            isLoading: false,
            result
        }
    }

</script>

{#if pageState.isLoading}
  <div class="p-6 text-center text-gray-400">Loading net table...</div>
{:else}
    <div class="flex flex-col h-full items-center">
        <header class="p-1 flex gap-2 items-center">
            Net Table <IconNetwork/>
            <input type="text" class="bg-transparent">
        </header>
        <main class="flex-1 w-full overflow-auto">
            {#each pageState.result.items as item}
                <div class="p-1">
                    {item.processId}
                    {item.processName}
                    {item.localIpAddress}
                    {item.localPort}
                    {item.remotePort}
                    {item.remoteIpAddress}
                </div>
            {/each}
        </main>
        <Pagination
            bind:page={pageState.page}
            bind:pageSize={pageState.pageSize}
            total={pageState.result.total}/>
    </div>
{/if}