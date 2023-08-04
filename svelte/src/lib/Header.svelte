<script lang="ts">
  import { addItem, searchItem, state } from "./stores";
</script>

<div
  class="sticky top-0 flex h-12 w-full items-center justify-center border-b bg-white py-2"
>
  <a class="no-underline" href="/">marks</a>
  &nbsp; &nbsp;
  <button on:click={addItem}
    ><i class="ri-add-circle-line" title="add new one" /></button
  >
  &nbsp; &nbsp;
  <input
    class="w-2/5 rounded-full border px-2 py-1 text-sm"
    bind:value={$state.query}
    placeholder="Search"
    on:keydown={(e) => e.key === "Enter" && searchItem()}
  />
</div>
