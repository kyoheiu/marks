<script lang="ts">
  import toast, { Toaster } from "svelte-french-toast";
  import { state } from "./stores";
  import { get } from "svelte/store";

  interface Payload {
    original: string;
    new: string;
    content: string;
  }

  const save = async () => {
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
      };
    });

    toast.success("Saved.", {
      duration: 2000,
    });
  };
</script>

<Toaster />
<button
  class="h-6 rounded-full bg-sky-500 px-2 text-sm text-white"
  on:click={save}><i class="ri-file-upload-line" title="save" /></button
>
