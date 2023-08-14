<script lang="ts">
  import {
    DEFAULT_LIST_NUMBER,
    editItemDirectly,
    readItem,
    state,
  } from "./stores";
  import DialogToDelete from "./DialogToDelete.svelte";
  import moment from "moment";

  const toDuration = (time: number) => {
    return moment.unix(time).fromNow(true);
  };
</script>

<div class="my-4 flex flex-col items-center rounded bg-item_background">
  {#each $state.filter ? $state.items.filter( (x) => x.name.includes($state.filter) ) : $state.seeMore ? $state.items : $state.items.slice(0, DEFAULT_LIST_NUMBER) as item}
    <div
      class="hover:bg-background flex h-16 w-64 list-none items-center border-b-2 border-background px-2 sm:w-120 md:w-144"
    >
      <span
        class="mr-4 h-4 w-4 rounded-full bg-background border-2 border-item_background"
      />
      <div class="flex flex-col items-start">
        <button
          on:click={() => !item.showModal && readItem(item.name)}
          class="w-36 truncate text-left sm:w-72 md:w-96">{item.name}</button
        >
        {#if item.desc}
          <div class="hidden w-40 truncate text-sm sm:inline sm:w-72 md:w-96">
            {item.desc}
          </div>
        {:else}
          <div class=" hidden w-40 truncate text-sm italic sm:inline sm:w-72 md:w-96">
            No contents.
          </div>
        {/if}
      </div>
      <span class="hidden w-12 text-right text-xs text-subtle sm:inline sm:w-16"
        >{toDuration(item.modified)}</span
      >
      <button
        class="mx-2 w-10 text-xs rounded border border-edit bg-item_background text-edit px-2 py-1"
        on:click={() => !item.showModal && editItemDirectly(item.name)}
        >Edit</button
      >
      <button
        class="w-6 text-xs"
        on:click={() => (item.showModal = true)}
        title="delete"
      >
        <i class="ri-close-line" />
      </button>
      <DialogToDelete bind:showModal={item.showModal} item={item.name} />
    </div>
  {/each}
</div>
