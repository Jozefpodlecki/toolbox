<script lang="ts">
    import { onMount } from "svelte";
    import { updateStatus } from "$lib/stores";

    onMount(() => {

    })

</script>

<footer class="bg-amber-950 w-full text-white p-2 text-xs">
  {#if $updateStatus.type === "idle"}
    <div></div>
  {:else if $updateStatus.type === "checking"}
    <div>Checking updates...</div>
  {:else if $updateStatus.type === "failed"}
    <div>Update failed: {$updateStatus.value}</div>
  {:else if $updateStatus.type === "newVersion"}
    <div>New version available: {$updateStatus.value}</div>
  {:else if $updateStatus.type === "downloading"}
    <div>
      Downloading version {$updateStatus.value.version}: {$updateStatus.value.length}/{$updateStatus.value.total} bytes
    </div>
  {:else if $updateStatus.type === "downloaded"}
    <div>Downloaded version {$updateStatus.value}</div>
  {:else if $updateStatus.type === "latest"}
    <div>You're on the latest version: {$updateStatus.version}</div>
  {/if}
</footer>