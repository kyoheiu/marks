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
  <div class="sticky top-12 flex w-64 sm:w-120 md:w-144 px-1 py-2 justify-center items-start bg-sky-200">
      <div class="grow break-all leading-5">
        {$state.fileName}
      </div>
      <button
        class="ml-2 box-border w-12 h-6 rounded-full bg-sky-500 hover:bg-sky-600 px-2 text-sm text-white"
        on:click={editItem} title="edit"><i class="ri-file-edit-line" /></button
      >
        <button
          class="ml-4"
          on:click={() => (showModal = true)}
          title="delete">
          <i class="ri-delete-bin-2-line text-zinc-500 hover:text-zinc-900 text-sm"></i>
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
