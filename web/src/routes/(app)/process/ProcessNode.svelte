<script lang="ts">
    import type { ProcessNode } from "$lib/types/index";
    import ProcessNodeItem from "./ProcessNode.svelte"

    interface Props {
        node: ProcessNode;
    }

    let { node }: Props = $props();

</script>

<div class="pl-3 border-l border-gray-500/30 py-1">
    <a href={`/process/${node.process.id}`} class="font-medium text-sm text-gray-200">
        {node.process.name} <span class="text-gray-400">#{node.process.id}</span>
    </a>

    {#each node.children as child}
        <div class="mt-1">
            <ProcessNodeItem node={child} />
        </div>
    {/each}
</div>
