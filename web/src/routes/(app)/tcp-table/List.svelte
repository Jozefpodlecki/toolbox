<script lang="ts">
    import { onMount } from "svelte";
    import { getTcpTable } from "$lib/api";
    import IconNetwork from "~icons/tabler/network";
    import type { PageArgs, Paged, TcpTableEntry } from "$lib/types";
    import Pagination from "$lib/components/Pagination.svelte";

    interface LoadedState extends PageArgs {
        pageSize: number;
        page: number;
        result: Paged<TcpTableEntry>;
    }

    type State =
    | ({ isLoading: true } & PageArgs)
    | ({ isLoading: false } & LoadedState);

    let pageState = $state<State>({
        pageSize: 10,
        page: 0,
        isLoading: true
    });

    onMount(() => {
       onLoad();
    })

    async function onLoad() {
        const args: PageArgs = {
            ...pageState
        };

       const result = await getTcpTable(args);

        pageState = {
            ...pageState,
            isLoading: false,
            result
        }
    }

</script>

{#if pageState.isLoading}
  <div class="p-6 text-center text-gray-400">Loading tcp table...</div>
{:else}
    <div class="flex flex-col h-full items-center">
        <header class="p-1 flex gap-2 items-center">
            Tcp Table <IconNetwork/>
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