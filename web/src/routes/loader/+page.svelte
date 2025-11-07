<script lang="ts">
    import { checkUpdates, loadApp, onUpdateStatusChange } from "$lib/api";
    import { updateStatus } from "$lib/stores";
    import type { UpdateStatus } from "$lib/types";
    import type { UnlistenFn } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import BlocksWave from "~icons/svg-spinners/blocks-wave";
    
    let subscriptions: Array<UnlistenFn> = [];

    onMount(() => {
        onLoad()
    });

    function onUpdateStatusChangeEvent(value: UpdateStatus) {
        updateStatus.set(value)
	}

    async function onLoad() {
        const result = await loadApp();

        subscriptions.push(await onUpdateStatusChange(onUpdateStatusChangeEvent));
        
        await checkUpdates(false)
    }

</script>

<section class="h-screen flex flex-col items-center justify-center" data-tauri-drag-region> 
    <main class="flex ">
        <img class="absolute inset-0 w-full h-full object-cover -z-10 brightness-10" alt="banner" src="banner.jpg" />
        <div class="flex flex-col items-center text-gray-300 text-lg text-center" data-tauri-drag-region>
            <BlocksWave font-size="30px"/>
            <span>{"Loading..."}</span>
        </div>
    </main>
</section>