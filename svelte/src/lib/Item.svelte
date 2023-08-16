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
      class="my-2 flex h-20 w-64 list-none items-center rounded bg-itembackground px-2 shadow sm:w-120 md:w-144"
    >
      <span class="inner mx-2 h-4 w-4 rounded-full bg-background" />
      <div class="ml-2 flex flex-col items-start">
        <button
          on:click={() => !item.showModal && readItem(item.name)}
          title="view this item"
          class="mb-2 w-32 cursor-pointer truncate text-left text-sm underline hover:text-hovertitle sm:w-72 sm:text-base md:w-96"
          >{item.name}</button
        >
        {#if item.desc}
          <div
            class="hidden truncate text-sm text-desc sm:inline sm:w-72 md:w-96"
          >
            {item.desc}
          </div>
        {:else}
          <div
            class="hidden truncate text-sm italic text-desc sm:inline sm:w-72 md:w-96"
          >
            No contents.
          </div>
        {/if}
      </div>
      <span class="ml-2 w-12 text-right text-xs text-subtle"
        >{toDuration(item.modified)}</span
      >
      <button
        class="ml-4 mr-2 hidden w-12 rounded border border-further px-1 py-1 text-xs text-warning sm:inline"
        on:click={() => (item.showModal = true)}
        title="delete"
      >
        Delete
      </button>
      <DialogToDelete bind:showModal={item.showModal} item={item.name} />
    </div>
  {/each}
</div>
