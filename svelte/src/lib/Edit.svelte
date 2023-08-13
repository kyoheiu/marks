<script lang="ts">
  import { state, reviewItem } from "./stores";
  import { get } from "svelte/store";
  import { toast, Toaster } from "svelte-french-toast";
  import { onDestroy } from "svelte";

  const ms = 2000;

  let edited = false;

  export const save = async () => {
    const s = get(state);
    if (s.newName === "" || !s.newName) {
      toast.error("File name required.", {
        duration: 2000,
      });
      return;
    }
    const res = await fetch(`/item`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        original: s.fileName,
        new: s.newName,
        content: s.content,
      }),
    });

    if (!res.ok) {
      const message = await res.text();
      toast.error(`Error occured: ${message}`, {
        duration: 2000,
      });
      return;
    }

    state.update((s) => {
      return {
        ...s,
        fileName: s.newName,
        content: s.content,
      };
    });

    edited = false;
  };

  const detectChange = () => {
    edited = true;
  };

  const keyDown = (e: KeyboardEvent) => {
    if (e.key === "Enter" && e.ctrlKey) {
      save();
      edited = false;
    }
  };

  const interval = setInterval(() => {
    if (get(state).newName && edited) {
      save();
      edited = false;
    }
  }, ms);

  onDestroy(() => clearInterval(interval));
</script>

<Toaster />
<div class="flex min-h-full flex-col items-center justify-center">
  <div class="sticky top-12 flex w-full items-center bg-sky-200 p-2">
    <input
      class="h-8 w-32 border border-zinc-300 px-2 font-mono text-sm sm:w-80 md:w-96"
      bind:value={$state.newName}
      placeholder="file name"
      on:input={detectChange}
      on:keydown={(e) => keyDown(e)}
    />
    <button
      class="ml-auto mr-2 box-border h-6 w-12 rounded-full border border-sky-500 bg-white px-2 text-sm text-sky-500 hover:bg-sky-500 hover:text-white"
      on:click={reviewItem}
      title="back to view"><i class="ri-arrow-go-back-line" /></button
    >
    <button
      class="box-border h-6 w-14 text-sm text-zinc-500"
      on:click={save}
      title="click / tap to manually save"
    >
      {#if !$state.newName && !$state.content && !edited}
        ...
      {:else if edited}
        <i class="ri-loop-right-line" />
      {:else}
        <i class="ri-check-line" /> saved
      {/if}
    </button>
  </div>
  <textarea
    class="h-120 w-64 flex-grow border border-zinc-300 bg-white p-3 font-mono text-sm text-zinc-900 outline-none sm:h-144 sm:w-120 md:w-144"
    contenteditable="true"
    bind:value={$state.content}
    placeholder="Write here."
    on:input={detectChange}
    on:keydown={(e) => keyDown(e)}
  />
</div>
