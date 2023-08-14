<script lang="ts">
  import Item from "./Item.svelte";
  import { state, seeMoreItem, DEFAULT_LIST_NUMBER } from "./stores";
</script>

<div class="flex flex-col items-center">
  {#if !$state.queryFixed}
    <input
      class="w-64 mt-4 px-2 py-1 text-sm shadow-inner"
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
  {#if $state.items.length > DEFAULT_LIST_NUMBER && !$state.filter && !$state.queryFixed && !$state.seeMore}
    <button
      class="mb-2 rounded border border-subtle text-subtle bg-item_background px-2 py-1 text-sm"
      on:click={seeMoreItem}
      title="show all">Show all</button
    >
  {/if}
</div>
