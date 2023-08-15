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

<div class="mt-4 flex justify-center text-sm">
  <div
    class="flex w-64 items-baseline justify-center px-1 pb-2 sm:w-120 md:w-144"
  >
    <div class="font-mono grow break-all leading-5">
      {$state.fileName}
    </div>
    <button
      class="ml-2 w-11 rounded bg-edit px-2 py-1 text-xs font-semibold text-item_background"
      on:click={editItem}
      title="edit">Edit</button
    >
    &nbsp; &nbsp;
    <button
      class="ml-auto text-xs text-subtle"
      on:click={() => (showModal = true)}
      title="delete"
    >
      Delete
    </button>
  </div>
  <DialogToDelete bind:showModal item={$state.fileName} />
</div>

<div class="flex min-h-full flex-col items-center">
  <div
    class="mb-6 mt-4 w-64 flex-grow break-words rounded-md bg-item_background p-3 font-mono text-sm sm:w-120 md:w-144"
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
