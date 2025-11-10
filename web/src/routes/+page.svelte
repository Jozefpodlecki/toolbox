<script lang="ts">
  import { onMount } from "svelte";
  import { getProgramsCount } from "$lib/api";

  let programsCount = 0;
  let activeProcesses = 0;
  let availableCommitted = "0 MB";
  let pagedMemory = "0 MB";
  let diskFree = "0 GB";

  onMount(async () => {
    await onLoad();
  });

  async function onLoad() {
    // Installed programs count
    programsCount = await getProgramsCount();

    // Active processes (mock example, implement API)
    activeProcesses = await fetchActiveProcesses();

    // Memory info (mock example)
    const mem = await fetchMemoryInfo();
    availableCommitted = mem.availableCommitted;
    pagedMemory = mem.pagedMemory;

    // Disk info (mock example)
    const disk = await fetchDiskInfo();
    diskFree = disk.freeSpace;
  }

  // Dummy async functions (replace with real APIs)
  async function fetchActiveProcesses() { return 127; }
  async function fetchMemoryInfo() {
    return { availableCommitted: "6.3 GB", pagedMemory: "2.1 GB" };
  }
  async function fetchDiskInfo() {
    return { freeSpace: "120 GB" };
  }
</script>

<div class="p-6 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
  <div class="bg-neutral-900 p-4 rounded-lg shadow flex flex-col items-center">
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
  </div>
</div>
