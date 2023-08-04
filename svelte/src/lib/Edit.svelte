<script lang="ts">
  import { state, reviewItem } from "./stores";
  import Upload from "./Upload.svelte";
  import { get } from "svelte/store";
  import { Toaster } from "svelte-french-toast";
  import { save } from "./Upload";

  let content = get(state).content;
</script>

<Toaster />
<div class="flex min-h-full flex-col items-center">
  <div class="flex justify-center">
    <div
      class="sticky top-12 flex w-full items-center bg-sky-200 p-2"
    >
      <input
        class="w-32 sm:w-80 md:w-96 h-8 border border-zinc-300 px-2 font-mono text-sm"
        bind:value={$state.newName}
        placeholder="file name"
        on:keydown={(e) => e.key === "Enter" && e.ctrlKey && save(content)}
      />
      <button
        class="box-border mx-2 w-12 h-6 rounded-full border border-sky-500 bg-white px-2 text-sm text-sky-500"
        on:click={reviewItem}
        title="back to view"><i class="ri-arrow-go-back-line" /></button
      >
      <Upload {content} />
    </div>
  </div>
  <textarea
    class="h-120 w-64 flex-grow border border-zinc-300 bg-white p-3 font-mono text-sm text-zinc-900 outline-none sm:h-144 sm:w-120 md:w-144"
    contenteditable="true"
    bind:value={content}
    placeholder="Write here."
    on:keydown={(e) => e.key === "Enter" && e.ctrlKey && save(content)}
  />
</div>
