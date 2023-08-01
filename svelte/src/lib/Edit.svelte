<script lang="ts">
  import { state, reviewItem } from "./stores";
  import Upload from "./Upload.svelte";
</script>

<div class="flex min-h-full flex-col">
  <div
    class="sticky top-12 grid max-w-full grid-cols-6 items-center gap-2 bg-sky-100 p-2"
  >
    <input
      class="col-span-4 h-8 border border-zinc-300 px-2 font-mono text-sm"
      bind:value={$state.newName}
      placeholder="file name"
    />
    <button
      class="h-6 rounded-full border border-sky-500 bg-white px-2 text-sm text-sky-500"
      on:click={reviewItem}
      title="preview"><i class="ri-arrow-go-back-line" /></button
    >
    <Upload />
  </div>
  <textarea
    class="w-72 h-120 md:h-144 flex-grow border border-zinc-300 bg-white p-2 font-mono text-sm text-zinc-900 outline-none sm:w-120 md:w-144"
    contenteditable="true"
    bind:innerText={$state.content}
    placeholder="Write here."
  />
  </div>
