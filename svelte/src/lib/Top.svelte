<script lang="ts">
  import Item from "./Item.svelte";
  import { state, seeMoreItem, DEFAULT_LIST_NUMBER } from "./stores";
</script>

<div class="flex flex-col items-center">
  {#if !$state.queryFixed}
    <input
      class="rounded-full w-48 mt-4 px-3 py-1 text-sm shadow-inner"
      bind:value={$state.filter}
      placeholder="Filter file name"
    />
  {/if}
  {#if $state.queryFixed}
    <div class="mt-2">Query: {$state.queryFixed}</div>
  {/if}
</div>
<div class="flex flex-col items-center">
  {#if $state.items}
    <Item />
  {/if}
  {#if $state.items.length > DEFAULT_LIST_NUMBER && !$state.filter && !$state.queryFixed && !$state.showAll}
    <button
      class="mb-2 rounded border-2 border-border text-subtle bg-item_background px-2 py-1 text-xs"
      on:click={seeMoreItem}
      title="show all">Show all</button
    >
  {/if}
</div>
