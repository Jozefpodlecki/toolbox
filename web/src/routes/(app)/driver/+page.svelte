<script lang="ts">
    import { onMount } from "svelte";
    import { getInstalledDrivers } from "$lib/api";

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
       const drivers = await getInstalledDrivers();

       console.log(drivers);
    }    

</script>

