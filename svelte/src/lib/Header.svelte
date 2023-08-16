<script lang="ts">
  import DialogToDelete from "./DialogToDelete.svelte";
  import Top from "./Top.svelte";
  import { Page, addItem, editItem, searchItem, state } from "./stores";

  let showModal = false;
</script>

<div
  class="sticky top-0 flex w-full flex-col items-center bg-itembackground text-sm shadow"
>
  <div class="flex h-12 w-64 items-baseline pt-2 sm:w-120 md:w-144">
    <a class=" no-underline" href="/">marks</a>
    &nbsp; &nbsp;
    {#if $state.page === Page.Top}
      <button
        on:click={addItem}
        class="w-12 rounded bg-basecolor px-2 py-1 text-xs font-semibold text-lightbuttontext"
        >+New</button
      >
    {:else}
      <button
        on:click={addItem}
        class="rounded border-2 border-baseborder bg-lightbuttontext px-2 py-1 text-xs font-semibold text-basecolor"
        >+New</button
      >
    {/if}
    &nbsp; &nbsp;
    <div class="relative ml-auto">
      <i class="ri-search-line absolute left-1 top-1 text-subtle" />
      <input
        name="search"
        class="w-32 rounded-full bg-background py-1 pl-6 pr-3"
        bind:value={$state.query}
        on:keydown={(e) => e.key === "Enter" && searchItem()}
      />
    </div>
  </div>
</div>
