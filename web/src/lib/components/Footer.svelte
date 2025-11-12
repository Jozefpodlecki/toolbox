<script lang="ts">
    import { onMount } from "svelte";
    import { writable } from "svelte/store";

    type UpdateStatus = {
        type: "idle"
    } | {
        type: "checking"
    } | {
        type: "downloading"
        version: string;
        value: number;
        total: number;
    } | {
        type: "downloaded";
        version: string;
    } | {
        type: "latest";
        version: string;
    }

    let updateState = writable<UpdateStatus>({
        type: "idle"
    });

    onMount(() => {

    })

    // setTimeout()

</script>

<footer class="bg-amber-950 w-full">
    <!-- <div class="p-1">Checking updates</div> -->
    <div class="p-1">Downloading version {}</div>
</footer>