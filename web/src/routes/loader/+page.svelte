<script lang="ts">
    import { checkUpdates, loadApp, onUpdateStatusChange } from "$lib/api";
    import { updateStatus } from "$lib/stores";
    import type { UpdateStatus } from "$lib/types";
    import type { UnlistenFn } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import BlocksWave from "~icons/svg-spinners/blocks-wave";
    import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

    let subscriptions: Array<UnlistenFn> = [];
    let state = $state({
        text: ""
    });

    onMount(() => {
        onLoad()
    });

    function onUpdateStatusChangeEvent(value: UpdateStatus) {
        switch(value.type) {
            case "idle":
                state.text = "Loading...";    
                break;
            case "checking":
                state.text = "Checking updates...";
                break;
            case "downloading":
                state.text = "Downloading...";
                break;
            case "downloaded":
                state.text = "Downloaded...";
                break;
            case "failed":
                state.text = "Could not connect to update server...";
                onContinue();
                break;
        }
	}

    async function onContinue() {
        const main = await WebviewWindow.getByLabel("main");
        if (!main) throw new Error("main window not found");
        await main.setAlwaysOnTop(true);
        await main.show();
        await main.setFocus();

        const current = getCurrentWebviewWindow();
        await current.close();
    }

    async function onLoad() {
        const result = await loadApp();

        updateStatus.subscribe(onUpdateStatusChangeEvent);
        subscriptions.push(await onUpdateStatusChange(updateStatus.set));
        
        await checkUpdates(false)
    }

</script>

<section class="h-screen flex flex-col items-center justify-center" data-tauri-drag-region> 
    <main class="flex ">
        <img class="absolute inset-0 w-full h-full object-cover -z-10 brightness-10" alt="banner" src="banner.jpg" />
        <div class="flex flex-col items-center text-gray-300 text-center" data-tauri-drag-region>
            <BlocksWave font-size="30px"/>
            <span class="text-sm mt-2">{state.text}</span>
        </div>
    </main>
</section>