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

<div class="my-4 flex flex-col items-center">
  {#each $state.filter ? $state.items.filter( (x) => x.name.includes($state.filter) ) : $state.showAll ? $state.items : $state.items.slice(0, DEFAULT_LIST_NUMBER) as item}
    <div
      class="my-2 flex h-16 w-64 list-none items-center rounded bg-item_background px-2 shadow sm:w-120 md:w-144"
    >
      <div class="flex flex-col items-start">
        <button
          on:click={() => !item.showModal && readItem(item.name)}
          title="view this item"
          class="w-36 cursor-pointer truncate text-left text-sm sm:w-72 sm:text-base md:w-96"
          >{item.name}</button
        >
        {#if item.desc}
          <div
            class="hidden w-40 truncate text-sm text-subtle sm:inline sm:w-72 md:w-96"
          >
            {item.desc}
          </div>
        {:else}
          <div
            class=" hidden w-40 truncate text-sm italic text-subtle sm:inline sm:w-72 md:w-96"
          >
            No contents.
          </div>
        {/if}
      </div>
      <span class="hidden w-12 text-right text-xs text-subtle sm:inline sm:w-16"
        >{toDuration(item.modified)}</span
      >
      <button
        class="ml-2 w-11 rounded border-2 border-base_border bg-item_background px-1 py-1 text-xs text-base_color sm:ml-4"
        on:click={() => !item.showModal && readItem(item.name)}
        title="edit">View</button
      >
      <button
        class="ml-2 w-11 rounded border-2 border-edit_border bg-item_background px-2 py-1 text-xs text-edit sm:ml-2"
        on:click={() => !item.showModal && editItemDirectly(item.name)}
        title="edit">Edit</button
      >
      <button
        class="ml-2 hidden w-4 text-xs text-subtle sm:inline"
        on:click={() => (item.showModal = true)}
        title="delete"
      >
        <i class="ri-close-line" />
      </button>
      <DialogToDelete bind:showModal={item.showModal} item={item.name} />
    </div>
  {/each}
</div>
