<script lang="ts">
    import { onScreenshotRequest, onUpdateStatusChange, saveScreenshot } from "$lib/api";
    import Footer from "$lib/components/Footer.svelte";
    import Header from "$lib/components/Header.svelte";
    import Navigation from "$lib/components/Navigation.svelte";
    import { updateStatus } from "$lib/stores";
    import type { UnlistenFn } from "@tauri-apps/api/event";
	import { onMount, type Snippet } from "svelte";
	import html2canvas from 'html2canvas-pro';

	interface Props {
		children?: Snippet;
	}

	let subscriptions: Array<UnlistenFn> = [];
	let { children }: Props = $props();

	onMount(() => {
        onLoad()
    });

	async function onScreenshot() {
		const canvas = await html2canvas(document.body);
		const dataUrl = canvas.toDataURL("image/png");
		// console.log(dataUrl);
		await saveScreenshot(dataUrl);
	}

	async function onLoad() {
		subscriptions.push(await onUpdateStatusChange(updateStatus.set));
		subscriptions.push(await onScreenshotRequest(onScreenshot));
	}

</script>

<div class="h-screen w-screen flex flex-row">
    <Navigation/>

    <div class="flex-1 flex flex-col">
        <Header/>

        <main class="flex-1 bg-gray-800 overflow-auto">
            {@render children?.()}
        </main>

        <Footer/>
    </div>
</div>

<style>
	:global(::-webkit-scrollbar) {
		width: 8px;
	}

	:global(::-webkit-scrollbar-thumb) {
		background-color: #4b5563;
		border-radius: 4px;
	}

	:global(::-webkit-scrollbar-thumb:hover) {
		background-color: #6b7280;
	}

	:global(::-webkit-scrollbar-track) {
		background-color: #1f2937;
	}
</style>