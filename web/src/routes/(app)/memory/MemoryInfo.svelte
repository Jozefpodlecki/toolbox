<script lang="ts">
    import { getMemoryInfo } from "$lib/api";
    import { onMount } from "svelte";

    interface MemoryInfo {
        info: any;
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
        pageState = {
            isLoading: false,
            info
        }
    }    

</script>

{JSON.stringify(pageState)};