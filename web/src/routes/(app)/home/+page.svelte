<script lang="ts">
    import { onMount } from "svelte";
    import WebSocket from '@tauri-apps/plugin-websocket'
    import { getProcesses } from "$lib/api";
    import ProcessNode from "./ProcessNode.svelte";
    import Navigation from "./Navigation.svelte";

    let pageState = $state({
        processes: new Array<any>()
    });

    onMount(() => {
       onLoad();
    })

    async function onLoad() {

        const processes = await getProcesses();

        pageState.processes = processes;

        // const ws = await WebSocket.connect('wss://example.com')

        // await ws.send('Hello World')

        // await ws.disconnect()
    }

</script>

<div data-tauri-drag-region class="h-screen w-screen flex">
    <Navigation/>
    <main class="flex-1 flex flex-col bg-amber-900">
        <header class="p-1">Toolbox</header>
        <main class="flex-1 bg-amber-950">
            
        </main>
    </main>
    <!-- {#each pageState.processes as node (node.process.id)}
         <ProcessNode node={node} />
    {/each} -->
</div>

