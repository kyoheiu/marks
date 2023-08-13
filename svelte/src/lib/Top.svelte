<script lang="ts">
  import Item from "./Item.svelte";
  import { state, seeMoreItem, DEFAULT_LIST_NUMBER } from "./stores";
</script>

<div class="flex flex-col items-center">
  {#if !$state.queryFixed}
    <input
      class="mt-4 px-2 py-1 text-sm shadow-inner"
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
      class="mb-2 rounded border border-base bg-item_background px-2 py-1 text-sm text-base"
      on:click={seeMoreItem}
      title="see more">See all</button
    >
  {/if}
</div>
