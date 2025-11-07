<script lang="ts">
  export let error: Error;
  export let status: number;
</script>

<div class="flex flex-col items-center justify-center min-h-screen bg-gray-100 text-gray-900 p-6">
  <h1 class="text-4xl font-bold mb-4">{status}</h1>
  <p class="text-xl mb-6">{error?.message ?? "Something went wrong."}</p>

  <a href="/home" 
     class="px-4 py-2 bg-indigo-600 text-white rounded hover:bg-indigo-500 transition">
     Back to Home
  </a>
</div>