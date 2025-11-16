<script lang="ts">
    import { minimize } from "$lib/api";
    import type { MouseEventHandler } from "svelte/elements";
    import IconMinus from "~icons/tabler/minus";

    const onMinimize: MouseEventHandler<HTMLElement> = async (event) => {
        await minimize();
    }

</script>

<header class="flex p-1 text-center bg-gray-900" data-tauri-drag-region>
    <span>Toolbox</span>
    <div class="ml-auto">
        <button type="button" onclick={onMinimize}><IconMinus/></button>
    </div>
</header>