<script lang="ts">
  import toast, {Toaster} from 'svelte-french-toast';

  export let fileName;
  export let content;

  interface Payload {
    name: string;
    body: string;
  }

  const save = async (payload: Payload) => {
    const _res = await fetch(`/item`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });
    toast.success("Saved.", {
      position: "bottom-center",
      duration: 2000
    });
  };
</script>

<Toaster />
<div class="flex justify-end">
  <button
    class="mx-4 mt-2"
    on:click={() => save({ name: fileName, body: content })}
    ><i class="ri-file-upload-line" /></button
  >
</div>
