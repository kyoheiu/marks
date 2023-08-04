<script lang="ts">
  import { get } from "svelte/store";
  import { state, editItem } from "./stores";
  import Showdown from "showdown";
  import DialogToDelete from "./DialogToDelete.svelte";
  const converter = new Showdown.Converter();

  let showModal = false;

  if (history.scrollRestoration) {
    history.scrollRestoration = "manual";
  }
</script>

<div class="flex min-h-full flex-col items-center">
  <div
    class="sticky top-12 flex w-64 items-start justify-center bg-sky-200 px-1 py-2 sm:w-120 md:w-144"
  >
    <div class="grow break-all leading-5">
      {$state.fileName}
    </div>
    <button
      class="ml-2 box-border h-6 w-12 rounded-full bg-sky-500 px-2 text-sm text-white hover:bg-sky-600"
      on:click={editItem}
      title="edit"><i class="ri-file-edit-line" /></button
    >
    <button class="ml-4" on:click={() => (showModal = true)} title="delete">
      <i
        class="ri-delete-bin-2-line text-sm text-zinc-500 hover:text-zinc-900"
      />
    </button>
    <DialogToDelete bind:showModal bind:item={$state.fileName} />
  </div>

  <div
    class="mb-6 w-64 flex-grow break-words rounded-md bg-white p-3 font-mono text-sm text-zinc-900 sm:w-120 md:w-144"
  >
    {#if $state.content.length === 0}
      <i>No contents.</i>
    {:else if $state.fileName.split(".").pop() === "md"}
      {@html converter.makeHtml(get(state).content)}
    {:else}
      {#each $state.content.split("\n") as line}
        {line}<br />
      {/each}
    {/if}
  </div>
</div>
