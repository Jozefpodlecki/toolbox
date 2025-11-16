<script lang="ts">
	import { onMount } from "svelte";
	import { getDashboardStats } from "$lib/api";
	import type { DashboardStats } from "$lib/types";

	type State = {
		isLoading: true;
	} | { isLoading: false; } & DashboardStats;

	let pageState = $state<State>({
		isLoading: true
	});

	onMount(() => {
		onLoad();
	});

	async function onLoad() {
    try {
      const stats = await getDashboardStats();

      console.log(stats);

      pageState = {
        isLoading: false,
        ...stats
      }; 
    } catch (error) {
      console.log(error);
    }
  }

</script>

{#if pageState.isLoading}
  <div class="p-6 text-center text-gray-400">Loading dashboard...</div>
{:else}
  <div class="p-6 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <div class="bg-neutral-900 p-4 rounded-lg shadow flex flex-col items-center">
      <div class="text-gray-400 text-sm">Installed Programs</div>
      <div class="text-3xl font-bold">{pageState.programsCount}</div>
    </div>

    <div class="bg-neutral-900 p-4 rounded-lg shadow flex flex-col items-center">
      <div class="text-gray-400 text-sm">Active Processes</div>
      <div class="text-3xl font-bold">{pageState.activeProcesses}</div>
    </div>

    <div class="bg-neutral-900 p-4 rounded-lg shadow flex flex-col items-center">
      <div class="text-gray-400 text-sm">Total Physical</div>
      <div class="text-3xl font-bold">{pageState.memory.totalPhysFormatted}</div>
    </div>

    <div class="bg-neutral-900 p-4 rounded-lg shadow flex flex-col items-center">
      <div class="text-gray-400 text-sm">Available Physical</div>
      <div class="text-3xl font-bold">{pageState.memory.availPhysFormatted}</div>
    </div>

    <div class="bg-neutral-900 p-4 rounded-lg shadow flex flex-col items-center">
      <div class="text-gray-400 text-sm">Memory Load</div>
      <div class="text-3xl font-bold">{pageState.memory.memoryLoad}%</div>
    </div>

    {#each pageState.disks as disk}
      <div class="bg-neutral-900 p-4 rounded-lg shadow col-span-full">
        <div class="text-gray-400 text-sm mb-2">Disk: {disk.model ?? "Unknown"} ({disk.diskType ?? "Unknown"})</div>
        <table class="w-full text-left border-collapse">
          <thead>
            <tr class="text-gray-400 text-sm border-b border-gray-700">
              <th class="py-1 px-2">Partition</th>
              <th class="py-1 px-2">FS Type</th>
              <th class="py-1 px-2">Total</th>
              <th class="py-1 px-2">Used</th>
              <th class="py-1 px-2">Free</th>
            </tr>
          </thead>
          <tbody>
            {#each disk.partitions as part}
              <tr class="border-b border-gray-800">
                <td class="py-1 px-2">{part.name}</td>
                <td class="py-1 px-2">{part.fsType ?? "-"}</td>
                <td class="py-1 px-2">{part.totalFormatted}</td>
                <td class="py-1 px-2">{part.usedFormatted}</td>
                <td class="py-1 px-2">{part.freeFormatted}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/each}
  </div>
{/if}