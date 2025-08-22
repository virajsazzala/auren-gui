<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { Input } from "flowbite-svelte";
  import { FileOutline } from 'flowbite-svelte-icons';

  let query = '';
  let results: string[] = [];
  let loading = false;
  let error: string | null = null;
  let searched = false;
  let lastQuery = '';

  // clear results if query is empty
  $: if (query.trim() === '') {
    results = [];
    searched = false;
    lastQuery = '';
    error = null;
    loading = false;
  }

  async function doSearch() {
    const q = query.trim();
    if (!q) return;

    searched = true;
    lastQuery = q;
    loading = true;
    error = null;

    try {
      results = await invoke<string[]>('search', { query: q });
    } catch (e) {
      error = 'Something went wrong. Please try again.';
      console.error(e);
      results = [];
    } finally {
      loading = false;
    }
  }

</script>
<div class="min-h-screen flex flex-col bg-gray-50">
  <main class="flex-1 flex flex-col items-center justify-start py-12 px-6 font-sans">
    <h1 class="text-2xl md:text-3xl font-semibold mb-6 text-slate-800 tracking-tight">Auren Search</h1>

    <!-- search -->
    <div class="w-full max-w-3xl mx-auto">
      <div class="flex items-center gap-3">
        <Input
          id="large-input"
          size="lg"
          placeholder="Type your query..."
          bind:value={query}
          disabled={loading}
          class="flex-grow"
        />
        <button
          on:click={doSearch}
          disabled={loading}
          class="bg-indigo-600 text-white px-6 py-3 text-lg font-semibold rounded-lg hover:bg-indigo-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-all"
        >
          {loading ? '...' : 'Go'}
        </button>
      </div>
    </div>

    {#if error}
      <p class="mt-6 text-red-600 text-lg">{error}</p>
    {/if}

    <!-- results -->
    <div class="w-full max-w-3xl mt-8">
      <div class="w-full">

        {#if loading}
          <div class="p-6 bg-white rounded-2xl shadow flex items-center justify-center">
            <svg class="w-5 h-5 animate-spin text-indigo-600" viewBox="0 0 24 24" fill="none"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"></path></svg>
            <span class="ml-3 text-gray-600">{#if lastQuery}Searching for "{lastQuery}"…{:else}Searching…{/if}</span>
          </div>
        {:else if results.length > 0}
          <div class="space-y-4">
            {#each results as result}
              <article class="bg-white rounded-xl border border-gray-100 shadow-sm hover:shadow-md transition p-4 flex gap-4 items-start dark:bg-slate-800 dark:border-slate-700">
                <div class="flex-none w-10 h-10 rounded-full bg-indigo-50 text-indigo-600 flex items-center justify-center">
                  <FileOutline class="w-5 h-5 text-indigo-600" aria-hidden="true" />
                </div>
                <div class="min-w-0">
                  <p class="text-slate-700 dark:text-slate-200 text-lg break-words">{result}</p>
                  <p class="mt-1 text-sm text-gray-500 dark:text-slate-400">metadata goes here.</p>
                </div>
              </article>
            {/each}
          </div>
        {/if}

      </div>
    </div>
  </main>

  <footer class="w-full">
    <div class="max-w-3xl mx-auto py-4 text-center text-sm text-slate-400">Made with ❤ Auren</div>
  </footer>
</div>
