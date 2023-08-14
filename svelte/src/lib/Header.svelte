<script lang="ts">
  import DialogToDelete from "./DialogToDelete.svelte";
  import { Page, addItem, editItem, searchItem, state } from "./stores";

  let showModal = false;
</script>

<div
  class="sticky top-0 flex w-full flex-col items-center bg-background text-sm shadow-md"
>
  <div class="flex h-12 w-64 items-baseline py-2 sm:w-120 md:w-144">
    <a class=" no-underline" href="/">marks</a>
    &nbsp; &nbsp;
    <button
      on:click={addItem}
      class="rounded border border-base_border bg-item_background px-2 py-1 text-xs text-base_color"
      >+New</button
    >
    &nbsp; &nbsp;
    <input
      class="ml-auto w-2/5 px-2 py-1 shadow-inner"
      bind:value={$state.query}
      placeholder="Search"
      on:keydown={(e) => e.key === "Enter" && searchItem()}
    />
  </div>

  {#if $state.page === Page.View}
    <div class="mt-3 sticky top-12 flex justify-center py-2 text-sm">
      <div class="flex w-64 items-baseline justify-center sm:w-120 md:w-144">
        <div class="grow break-all leading-5">
          {$state.fileName}
        </div>
        <button
          class="rounded w-11 text-xs bg-edit px-2 py-1 text-item_background"
          on:click={editItem}
          title="edit">Edit</button
        >
        &nbsp; &nbsp;
        <button
          class="ml-auto text-xs text-subtle"
          on:click={() => (showModal = true)}
          title="delete"
        >
          Delete
        </button>
      </div>
      <DialogToDelete bind:showModal item={$state.fileName} />
    </div>
  {/if}
</div>
