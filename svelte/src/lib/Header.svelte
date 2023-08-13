<script lang="ts">
  import View from "./View.svelte";
  import { Page, addItem, editItem, searchItem, state } from "./stores";
</script>

<div class="sticky top-0 w-full text-sm shadow-md">
  <div class="flex h-12 items-center justify-center border-b bg-gray-200 py-2">
    <a class="no-underline" href="/">marks</a>
    &nbsp; &nbsp;
    <button on:click={addItem} class="bg-gray-600 px-2 py-1 text-gray-50"
      >New</button
    >
    &nbsp; &nbsp;
    <input
      class="w-2/5 rounded-full border px-2 py-1 shadow-inner"
      bind:value={$state.query}
      placeholder="Search"
      on:keydown={(e) => e.key === "Enter" && searchItem()}
    />
  </div>

  {#if $state.page === Page.View}
    <div class="sticky top-12 flex justify-center bg-gray-200 py-2">
      <div class="flex w-64 items-baseline justify-center sm:w-120 md:w-144">
        <div class="grow break-all leading-5">
          {$state.fileName}
        </div>
        <button
          class="bg-gray-600 px-2 py-1 text-gray-50"
          on:click={editItem}
          title="edit">Edit</button
        >
      </div>
    </div>
  {/if}
</div>
