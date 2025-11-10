<script lang="ts">
    import type { Snippet } from "svelte";
    import { createTooltip, melt } from '@melt-ui/svelte';
    import { fade } from 'svelte/transition';

    interface Props {
		children?: Snippet;
        text: String;
	}

	let { children, text }: Props = $props();

    const {
        elements: { trigger, content, arrow },
        states: { open },
    } = createTooltip({
        positioning: {
        placement: 'top',
        },
        openDelay: 0,
        closeDelay: 0,
        closeOnPointerDown: false,
        forceVisible: true,
    });
</script>

<div class="trigger" use:melt={$trigger} aria-label="Add">
     {@render children?.()}
</div>

{#if $open}
	<div
	use:melt={content}
	transition:fade={{ duration: 100 }}
	class="absolute z-10 rounded-lg bg-black text-white px-2 py-1 text-xs shadow"
	>
	{text}
	<div use:melt={arrow} class="tooltip-arrow" />
	</div>
{/if}

<style>

    .trigger {
        display: inline;
    }

    .tooltip-arrow {
        width: 0;
        height: 0;
        border-left: 5px solid transparent;
        border-right: 5px solid transparent;
        border-top: 5px solid black;
        margin: 0 auto;
    }
</style>