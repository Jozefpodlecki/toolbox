<script lang="ts">
	import type { Snippet } from "svelte";
	import "../app.css";
    import Navigation from "./(app)/home/Navigation.svelte";

	interface Props {
		children?: Snippet;
	}

	let { children }: Props = $props();


</script>

<div data-tauri-drag-region class="h-screen w-screen flex">
    <Navigation/>
    <main class="flex-1 flex flex-col bg-gray-900">
        <header data-tauri-drag-region class="p-1 text-center">Toolbox</header>
        <main data-tauri-drag-region class="flex-1 bg-gray-800 overflow-auto">
			{@render children?.()}
        </main>
    </main>
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