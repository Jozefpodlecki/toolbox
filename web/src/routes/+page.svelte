<script lang="ts">
  import { onMount } from "svelte";
  import { getDashboardStats, getProgramsCount } from "$lib/api";
    import type { DiskInfo, MemoryInfo } from "$lib/types";

	type State = {
		isLoading: true;
	} | {
		isLoading: false;
		programsCount: number;
		activeProcesses: number;
		memory: MemoryInfo;
		disk: DiskInfo[];
	}

  let pageState = $state<State>({
    isLoading: true
  });

  onMount(() => {
    onLoad();
  });

  async function onLoad() {
    const stats = await getDashboardStats();

    pageState = {
      isLoading: false,
      ...stats
    };

  }

  // // Dummy async functions (replace with real APIs)
  // async function fetchActiveProcesses() { return 127; }
  // async function fetchMemoryInfo() {
  //   return { availableCommitted: "6.3 GB", pagedMemory: "2.1 GB" };
  // }
  // async function fetchDiskInfo() {
  //   return { freeSpace: "120 GB" };
  // }
</script>

<div class="p-6 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
  <!-- <div class="bg-neutral-900 p-4 rounded-lg shadow flex flex-col items-center">
    <div class="text-gray-400 text-sm">Installed Programs</div>
    <div class="text-3xl font-bold">{programsCount}</div>
  </div>

  <div class="bg-neutral-900 p-4 rounded-lg shadow flex flex-col items-center">
    <div class="text-gray-400 text-sm">Active Processes</div>
    <div class="text-3xl font-bold">{activeProcesses}</div>
  </div>

  <div class="bg-neutral-900 p-4 rounded-lg shadow flex flex-col items-center">
    <div class="text-gray-400 text-sm">Available Committed</div>
    <div class="text-3xl font-bold">{availableCommitted}</div>
  </div>

  <div class="bg-neutral-900 p-4 rounded-lg shadow flex flex-col items-center">
    <div class="text-gray-400 text-sm">Paged Memory</div>
    <div class="text-3xl font-bold">{pagedMemory}</div>
  </div>

  <div class="bg-neutral-900 p-4 rounded-lg shadow flex flex-col items-center">
    <div class="text-gray-400 text-sm">Disk Free</div>
    <div class="text-3xl font-bold">{diskFree}</div>
  </div> -->
</div>
