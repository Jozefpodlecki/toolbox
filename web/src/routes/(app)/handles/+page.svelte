<script lang="ts">
    import { onMount } from "svelte";
    import { getSystemHandles } from "$lib/api";

    interface MemoryInfo {

    }

    type State = { isLoading: true } | { isLoading: false } & MemoryInfo;

    let pageState = $state<State>({
        isLoading: true
    });

    onMount(() => {
       onLoad();
    })

    async function onLoad() {
       const handles = await getSystemHandles();

       console.log(handles);
    }    

</script>

