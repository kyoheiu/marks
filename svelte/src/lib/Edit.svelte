<script lang="ts">
  import { state, reviewItem } from "./stores";
  import Upload from "./Upload.svelte";
  import { get } from "svelte/store";

  let content = get(state).content;
</script>

<div class="flex min-h-full flex-col items-center">
  <div class="flex justify-center">
    <div
      class="sticky top-12 grid w-5/6 grid-cols-6 items-center gap-2 bg-sky-100 p-2"
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
      <Upload {content} />
    </div>
  </div>
  <textarea
    class="h-120 w-64 flex-grow border border-zinc-300 bg-white p-2 font-mono text-sm text-zinc-900 outline-none sm:h-144 sm:w-120 md:w-144"
    contenteditable="true"
    bind:value={content}
    placeholder="Write here."
  />
</div>
