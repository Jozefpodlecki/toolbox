<script lang="ts">
    import { onMount } from "svelte";
    import { getInstalledDrivers } from "$lib/api";

    interface DriverInfo {
        drivers?: any;
        error?: any
    }

    type State = { isLoading: true } | { isLoading: false } & DriverInfo;

    let pageState = $state<State>({
        isLoading: true
    });

    onMount(() => {
       onLoad();
    })

    async function onLoad() {
       
        try {
            const drivers = await getInstalledDrivers();

            pageState = {
                isLoading: false,
                drivers
            }   
        } catch (error) {
            pageState = {
                isLoading: false,
                error
            }
        }
    }    

</script>

{JSON.stringify(pageState)};