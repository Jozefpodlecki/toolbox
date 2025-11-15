<script lang="ts">
    import { onUpdateStatusChange } from "$lib/api";
    import { updateStatus } from "$lib/stores";
    import type { UnlistenFn } from "@tauri-apps/api/event";
	import { onMount, type Snippet } from "svelte";

	interface Props {
		children?: Snippet;
	}

	let subscriptions: Array<UnlistenFn> = [];
	let { children }: Props = $props();

	onMount(() => {
        onLoad()
    });

	async function onLoad() {
		subscriptions.push(await onUpdateStatusChange(updateStatus.set));
	}

</script>

{@render children?.()}
