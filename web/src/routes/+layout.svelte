<script lang="ts">
	import type { Snippet } from "svelte";
	import "../app.css";
    import Navigation from "../lib/components/Navigation.svelte";
    import Footer from "$lib/components/Footer.svelte";

	interface Props {
		children?: Snippet;
	}

	let { children }: Props = $props();


</script>

<div class="h-screen w-screen flex flex-row">
    <Navigation/>

    <div class="flex-1 flex flex-col">
        <header class="p-2 text-center bg-gray-900" data-tauri-drag-region>Toolbox</header>

        <main class="flex-1 bg-gray-800 overflow-auto" data-tauri-drag-region>
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