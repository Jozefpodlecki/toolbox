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

{@render children?.()}