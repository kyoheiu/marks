<script lang="ts">
  import { get } from "svelte/store";
  import { state, editItem } from "./stores";
  import Showdown from "showdown";
  import DialogToDelete from "./DialogToDelete.svelte";
  const converter = new Showdown.Converter();

  let showModal = false;
</script>

<div class="flex min-h-full flex-col">
  <div
    class="sticky top-12 grid max-w-full grid-cols-6 items-center gap-2 bg-sky-100 p-2"
  >
    <div class="col-span-4 truncate text-lg">
      {$state.fileName}
    </div>
    <button
      class="h-6 rounded-full bg-sky-500 px-2 text-sm text-white"
      on:click={editItem}><i class="ri-file-edit-line" /></button
    >
    <button
      class="h-6 rounded-full border border-red-400 bg-white px-2 text-sm text-red-400"
      on:click={() => (showModal = true)}
      ><i class="ri-delete-bin-2-line" /></button
    >
    <DialogToDelete bind:showModal item={$state.newName} />
  </div>

  <div
    class="w-72 flex-grow rounded-md bg-white p-2 font-mono text-sm text-zinc-900 sm:w-120 md:w-144"
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
