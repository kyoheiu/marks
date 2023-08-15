<script lang="ts">
  import Item from "./Item.svelte";
  import { state, seeMoreItem, DEFAULT_LIST_NUMBER } from "./stores";
</script>

<div class="flex flex-col items-center">
  {#if !$state.queryFixed}
    <input
      class="mt-4 w-48 rounded-full px-3 py-1 text-sm"
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
      class="mb-4 rounded bg-further px-2 py-1 text-xs"
      on:click={seeMoreItem}
      title="show all">Show all</button
    >
  {/if}
</div>
