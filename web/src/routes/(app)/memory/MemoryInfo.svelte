<script lang="ts">
    import { getMemoryInfo } from "$lib/api";
    import { onMount } from "svelte";

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
       const info = await getMemoryInfo();

       console.log(info);
    }    

</script>

