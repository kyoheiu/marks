<script lang="ts">
  import DialogToDelete from "./DialogToDelete.svelte";
  import { state, readItem, seeMoreItem } from "./stores";
</script>

<div class="flex flex-col items-center">
  {#if !$state.queryFixed}
    <input
      class="mb-2 mt-4 rounded-full border p-1 text-sm"
      bind:value={$state.filter}
      placeholder="filter"
    />
  {/if}
  {#if $state.queryFixed}
    <div class="m-2">Query: {$state.queryFixed}</div>
  {/if}
</div>
<div class="flex flex-col items-center">
  {#if $state.items}
    {#each $state.filter ? $state.items.filter( (x) => x.name.includes($state.filter) ) : $state.seeMore ? $state.items : $state.items.slice(0, 5) as item}
      <div
        class="my-2 flex w-48 items-center justify-between border-b border-zinc-800"
      >
        <button
          class="truncate text-sm font-semibold"
          on:click={() => !item.showModal && readItem(item.name)}
          title="view">{item.name}</button
        >
        &nbsp;
        <button on:click={() => (item.showModal = true)}
          ><i
            class="ri-delete-bin-2-line text-sm text-zinc-600"
            title="delete"
          /></button
        >
      </div>
      <DialogToDelete bind:showModal={item.showModal} {item} />
    {/each}
  {/if}
  {#if !$state.filter && !$state.queryFixed && !$state.seeMore}
    <button class="m-2" on:click={seeMoreItem} title="see more"
      ><i class="ri-more-2-line" /></button
    >
  {/if}
</div>
