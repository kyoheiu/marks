<script lang="ts">
  import { state, reviewItem } from "./stores";
  import { get } from "svelte/store";
  import { toast, Toaster } from "svelte-french-toast";
  import { onDestroy } from "svelte";

  const ms = 2000;

  let edited = false;

  export const save = async () => {
    const s = get(state);
    const fileName = s.fileName;
    const newName = s.newName;
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
        original: fileName,
        new: newName,
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
        fileName: newName,
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
    const s = get(state);
    if (s.newName && edited) {
      save();
      edited = false;
    }
  }, ms);

  onDestroy(() => clearInterval(interval));
</script>

<Toaster />
<div class="mt-2 flex min-h-full flex-col items-center justify-center">
  <div class="sticky top-12 flex w-full items-center p-2">
    <input
      class="h-8 w-32 px-2 font-mono text-sm shadow-inner sm:w-64 md:w-96"
      bind:value={$state.newName}
      placeholder="file name"
      on:input={detectChange}
      on:keydown={(e) => keyDown(e)}
    />
    <button
      class="ml-auto mr-2 rounded bg-base px-2 py-1 text-sm text-item_background"
      on:click={reviewItem}
      title="back to view">View</button
    >
    <button
      class="h-6 w-6 text-sm text-subtle"
      on:click={save}
      title="click / tap to manually save"
    >
      {#if !$state.newName && !$state.content && !edited}
        ...
      {:else if edited}
      <i class="ri-checkbox-blank-circle-fill text-unsaved" />
      {:else}
      <i class="ri-checkbox-circle-fill text-saved" />
      {/if}
    </button>
  </div>
  <textarea
    class="h-120 w-64 flex-grow p-3 font-mono text-sm shadow-inner outline-none sm:h-144 sm:w-120 md:w-144"
    contenteditable="true"
    bind:value={$state.content}
    placeholder="Write here."
    on:input={detectChange}
    on:keydown={(e) => keyDown(e)}
  />
</div>
