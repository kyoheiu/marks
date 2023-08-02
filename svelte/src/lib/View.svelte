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
  <div class="sticky top-12 flex w-full items-center justify-center bg-sky-200">
    <div class="grid w-64 grid-cols-7 gap-2 px-1 py-2 sm:w-120 md:w-144">
      <div class="col-span-4 truncate">
        {$state.fileName}
      </div>
      <button
        class="col-span-2 box-border h-6 rounded-full bg-sky-500 px-2 text-sm text-white"
        on:click={editItem}><i class="ri-file-edit-line" /></button
      >
      <button
        class="box-border h-6 rounded-full border border-red-400 bg-white px-2 text-sm text-red-400"
        on:click={() => (showModal = true)}
        ><i class="ri-delete-bin-2-line" /></button
      >
      <DialogToDelete bind:showModal bind:item={$state.fileName} />
    </div>
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
