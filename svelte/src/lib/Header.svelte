<script lang="ts">
  import DialogToDelete from "./DialogToDelete.svelte";
    import Top from "./Top.svelte";
  import { Page, addItem, editItem, searchItem, state } from "./stores";

  let showModal = false;
</script>

<div
  class="sticky top-0 flex w-full flex-col items-center bg-item_background text-sm shadow"
>
  <div class="flex h-12 w-64 items-baseline pt-2 sm:w-120 md:w-144">
    <a class=" no-underline" href="/">marks</a>
    &nbsp; &nbsp;
    {#if $state.page === Page.Top}
    <button
      on:click={addItem}
      class="rounded bg-base_color px-2 py-1 text-xs font-semibold text-item_background"
      >+New</button
    >
    {:else}
    <button
      on:click={addItem}
      class="rounded border-2 border-base_border text-base_color px-2 py-1 text-xs font-semibold bg-item_background"
      >+New</button
    >
    {/if}
    &nbsp; &nbsp;
    <div class="relative ml-auto">
      <i class="ri-search-line absolute left-1 top-1 text-border" />
      <input
        class="w-32 rounded-full bg-background py-1 pl-6 pr-3"
        bind:value={$state.query}
        on:keydown={(e) => e.key === "Enter" && searchItem()}
      />
    </div>
  </div>
</div>
