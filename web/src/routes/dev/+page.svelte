<script lang="ts">
    import { requestScreenshot } from "$lib/api";
    import type { MouseEventHandler } from "svelte/elements";
    import IconCamera from "~icons/tabler/camera";

    const onScreenshot: MouseEventHandler<HTMLElement> = async (event) => {
        await requestScreenshot();
    }

</script>

<section class="h-screen w-screen flex flex-col">
    <header data-tauri-drag-region class="text-center text-sm bg-gray-900">Dev</header>
    <main class="p-1">
        <button type="button" onclick={onScreenshot} class="flex items-center gap-1">
            <IconCamera/> <span class="text-xs">Take screenshot</span>
        </button>
    </main>
</section>