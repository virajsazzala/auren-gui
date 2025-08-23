<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    Input,
    Label,
    Button,
    InputAddon,
    ButtonGroup,
  } from "flowbite-svelte";
  import { Card } from "flowbite-svelte";
  import {
    GiftBoxSolid,
    ArrowUpRightFromSquareOutline,
  } from "flowbite-svelte-icons";
  import { FileOutline, SearchOutline } from "flowbite-svelte-icons";

  let query = "";
  let results: string[] = [];
  let loading = false;
  let error: string | null = null;
  let searched = false;
  let preview = false;
  let currentPreview = "";

  $: if (query.trim() === "") {
    results = [];
    searched = false;
    error = null;
    loading = false;
  }

  async function doTest() {
    console.log("Pressed");
  }

  async function doSearch() {
    const q = query.trim();
    if (!q) return;

    searched = true;
    loading = true;
    error = null;

    try {
      results = await invoke<string[]>("search", { query: q });
    } catch (e) {
      error = "Something went wrong. Please try again.";
      console.error(e);
      results = [];
    } finally {
      loading = false;
    }
  }

  async function loadPreview(result: string) {
    currentPreview = result;
    preview = true;
  }

  async function clearPreview() {
    currentPreview = "";
    preview = false;
  }
</script>

<main class="min-h-screen px-16 pt-20">
  <div class="flex flex-col space-y-4 w-full max-w-3xl mx-auto">
    <h1 class="text-2xl md:text-3xl font-semibold text-center">Auren</h1>
    <div class="flex flex-row gap-4 w-full">
      <Input
        id="large-input"
        size="md"
        placeholder="What would you like to find today?"
        bind:value={query}
        disabled={loading}
        class="flex-grow"
      />
      <button
        on:click={doSearch}
        disabled={loading}
        class="bg-indigo-600 rounded-lg"
      >
        <Button class="!p-2">
          {#if loading}
            <SearchOutline class="h-6 w-6" />
          {:else}
            <SearchOutline class="h-6 w-6" />
          {/if}
        </Button>
      </button>
    </div>

    <div class="flex flex-row items-start gap-4">
      {#if results.length > 0}
        <!-- {:else if results.length > 0} -->
        <div class="w-1/2 space-y-3">
          {#each results as result}
            <div
              role="region"
              on:mouseenter={() => loadPreview(result)}
              on:mouseleave={clearPreview}
            >
              <Card
                class="max-w-full flex flex-row items-center w-full h-15 p-3 gap-3"
              >
                <GiftBoxSolid
                  class="h-6 w-6 text-gray-500 dark:text-gray-400"
                />
                <a href="/">
                  <h5
                    class="text-xl font-semibold tracking-tight text-gray-900 dark:text-white"
                  >
                    {result}
                  </h5>
                </a>
              </Card>
            </div>
          {/each}
        </div>
      {:else}
        <div class="w-1/2">
          <p class="text-gray-500"></p>
        </div>
      {/if}

      <div class="w-1/2 p-6 {preview ? 'border' : ''}">
        {#if preview}
          {currentPreview}: Lorem ipsum dolor sit amet, consectetur adipiscing
          elit. Nam in tortor at dui blandit tempor. Nunc blandit nisi at
          tincidunt tristique. Mauris fermentum aliquam sem et sodales. Sed
          vitae dui nec lorem rutrum malesuada non vitae diam. Duis euismod
          feugiat facilisis. Nulla eu purus id justo tempus mollis sit amet et
          risus. Mauris vitae ex in enim placerat ullamcorper. Nunc eros quam,
          pellentesque eu euismod ut, dictum sit amet nulla. Aenean convallis
          eros quis leo aliquam mattis. Aenean ac commodo tellus. Donec dui
          sapien, dictum in bibendum eu, molestie vitae augue. Nullam a lectus
          maximus elit luctus blandit ut id risus. Morbi dui ex, cursus nec
          ultricies id, faucibus sed ligula. Praesent porta est tortor, vel
          ultricies nisl dictum eu. Cras feugiat consectetur aliquam. Duis
          volutpat nunc at felis pellentesque, euismod tincidunt lorem
          malesuada. Mauris ac porta dui, id porttitor nulla. Praesent vitae
          risus posuere elit malesuada cursus sit amet vel felis. Vestibulum
          feugiat at ipsum et sagittis. Duis sit amet vulputate turpis. In
          vehicula, lacus nec tristique convallis, elit purus pretium purus, non
          ornare turpis metus ut est. Aliquam erat volutpat. Ut quis porta
          lectus. Phasellus nec sodales elit, sed porttitor nunc. Maecenas eget
          diam eros. Vestibulum ac nisl nec purus accumsan tempus.
        {/if}
      </div>
    </div>
  </div>
</main>
