<script lang="ts">
    import { onMount } from "svelte";
    import WebSocket from '@tauri-apps/plugin-websocket'
    import Navigation from "./Navigation.svelte";
    import Processes from "../process/Processes.svelte";

    let pageState = $state({
        
    });

    onMount(() => {
       onLoad();
    })

    async function onLoad() {

        // const ws = await WebSocket.connect('wss://example.com')

        // await ws.send('Hello World')

        // await ws.disconnect()
    }

</script>

<!-- <div data-tauri-drag-region class="h-screen w-screen flex">
    <Navigation/>
    <main class="flex-1 flex flex-col bg-gray-900">
        <header data-tauri-drag-region class="p-1 text-center">Toolbox</header>
        <main data-tauri-drag-region class="flex-1 bg-gray-800 overflow-auto">
            <Processes/>
        </main>
    </main>
</div>
 -->
