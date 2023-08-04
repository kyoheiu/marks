<script lang="ts">
  import DialogToDelete from "./DialogToDelete.svelte";
  import Item from "./Item.svelte";
  import { state, readItem, seeMoreItem, DEFAULT_LIST_NUMBER } from "./stores";
</script>

<div class="flex flex-col items-center">
  {#if !$state.queryFixed}
    <input
      class="mb-2 mt-4 w-5/6 rounded-full border py-1 px-2 text-sm"
      bind:value={$state.filter}
      placeholder="Filter file name"
    />
  {/if}
  {#if $state.queryFixed}
    <div class="m-2">Query: {$state.queryFixed}</div>
  {/if}
</div>
<div class="flex flex-col items-center">
  {#if $state.items}
    <Item />
  {/if}
  {#if $state.items.length > DEFAULT_LIST_NUMBER && !$state.filter && !$state.queryFixed && !$state.seeMore}
    <button
      class="m-2 rounded-md bg-white p-2 text-sm text-zinc-500"
      on:click={seeMoreItem}
      title="see more">See all</button
    >
  {/if}
</div>
