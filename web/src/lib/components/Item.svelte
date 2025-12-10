<script lang="ts">
    import { page } from "$app/state";
    import type { Snippet } from "svelte";
    import type { MouseEventHandler } from "svelte/elements";

    interface Props {
        href?: string;
        onclick?: MouseEventHandler<HTMLElement>;
        children?: Snippet;
        className?: string;
    }

    let { href, onclick, className, children }: Props = $props();

</script>

{#if href}
  <a href={href}
    class={`flex gap-1 items-center p-1 hover:bg-gray-700 group ${className ?? ""}`}
    data-selected={page.url.pathname === href} >
    {@render children?.()}
  </a>
{:else if onclick}
  <button type="button" class={`flex gap-1 p-1 ${className ?? ""}`} onclick={onclick} >
    {@render children?.()}
  </button>
{/if}