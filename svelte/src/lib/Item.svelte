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
    <div class="h-30 w-60 rounded-md bg-white p-2">
      <div class="flex items-start justify-between border-b">
        <button
          class="w-52 truncate text-left text-lg font-medium"
          on:click={() => !item.showModal && readItem(item.name)}
          title="view">{item.name}</button
        >
        <button
          class="text-zinc-500"
          on:click={() => (item.showModal = true)}
          title="delete"> 
          <i class="ri-close-circle-fill text-zinc-500 text-xs"></i>
          </button>
      </div>
      <div class="mb-4 line-clamp-2 h-8 py-1 font-mono text-xs text-zinc-500">
        {#if item.desc}
          {item.desc}
        {:else}
          <i>No contents.</i>
        {/if}
      </div>
      <div class="bottom-1 flex w-56 items-end">
        <span class="text-xs text-zinc-500">
          {toDuration(item.modified)}
        </span>
        {#if item.name.split(".").pop() === "md"}
          <i class="ml-auto ri-markdown-fill text-sky-500" />
        {:else}
        &nbsp;
        {/if}
      </div>
      <DialogToDelete bind:showModal={item.showModal} item={item.name} />
    </div>
  {/each}
</div>
