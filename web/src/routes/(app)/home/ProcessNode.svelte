<script lang="ts">
    import type { ProcessNode } from "$lib/types";

    interface Props {
        node: ProcessNode;
    }

    let { node }: Props = $props();

</script>

<div>
    {node.process.name}
</div>