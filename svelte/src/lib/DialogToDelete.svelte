<script>
  import { get } from "svelte/store";
  import { state } from "./stores";
  import toast, { Toaster } from "svelte-french-toast";

  export let showModal; // boolean
  export let item; // string
  let dialog; // HTMLDialogElement

  const deleteItem = async () => {
    const res = await fetch("/item", {
      method: "DELETE",
      body: item,
    });
    if (!res.ok) {
      dialog.close();
      const message = await res.text();
      toast.error(`Error occured: ${message}`, {
        duration: 2000,
      });
      return;
    }
    dialog.close();
    window.location.assign("/");
  };

  $: if (dialog && showModal) dialog.showModal();
</script>

<Toaster />
<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<div class="m-auto">
  <dialog
    class="w-7/8 max-w-sm rounded-md"
    bind:this={dialog}
    on:close={() => (showModal = false)}
    on:click|self={() => dialog.close()}
  >
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div on:click|stopPropagation>
      <div class="mb-4 break-all p-2">
        Are you sure to delete <code class="bg-zinc-200 rounded-md p-1"
          >{item}</code
        >?<br />
        This action is irreversible.
      </div>
      <hr class="border-zinc-900 my-4" />
      <!-- svelte-ignore a11y-autofocus -->
      <div class="flex justify-between">
        <button
          class="border-zinc-900 hover:bg-zinc-900 hover:text-white rounded-md border-2 px-1 text-sm font-semibold"
          on:click={() => dialog.close()}>cancel</button
        >
        <button
          class="border-red-500 text-red-500 hover:bg-red-500 hover:text-white rounded-md border-2 px-1 text-sm font-semibold"
          on:click={() => deleteItem()}>delete</button
        >
      </div>
    </div>
  </dialog>
</div>
