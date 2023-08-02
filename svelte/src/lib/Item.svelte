<script lang="ts">
  import { DEFAULT_LIST_NUMBER, readItem, state } from "./stores";
  import DialogToDelete from "./DialogToDelete.svelte";
  import moment from "moment";

  const toDuration = (time: number) => {
    return moment.unix(time).fromNow();
  };
</script>

<div class="grid grid-cols-1 gap-2 p-2 sm:grid-cols-2">
  {#each $state.filter ? $state.items.filter( (x) => x.name.includes($state.filter) ) : $state.seeMore ? $state.items : $state.items.slice(0, DEFAULT_LIST_NUMBER) as item}
    <div class="relative h-32 w-60 rounded-md bg-white p-2">
      <button
        class="w-56 truncate text-left text-lg font-semibold"
        on:click={() => !item.showModal && readItem(item.name)}
        title="view">{item.name}</button
      >
      {#if item.desc}
        <div class="line-clamp-2 py-1 font-mono text-sm text-zinc-500">
          {item.desc}
        </div>
      {:else}
        <div class="line-clamp-2 py-1 font-mono text-sm text-zinc-500">
          <i>No contents.</i>
        </div>
      {/if}
      <div class="absolute bottom-1 flex w-56 items-center justify-between">
        <span class="text-xs text-zinc-600">
          {toDuration(item.modified)}
        </span>
        <button
          class="text-zinc-600"
          on:click={() => (item.showModal = true)}
          title="delete"><i class="ri-delete-bin-2-line text-sm" /></button
        >
      </div>
      <DialogToDelete bind:showModal={item.showModal} item={item.name} />
    </div>
  {/each}
</div>
