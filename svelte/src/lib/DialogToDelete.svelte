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
        Are you sure to delete <code class="rounded-md bg-zinc-200 p-1"
          >{item}</code
        >?<br />
        This action is irreversible.
      </div>
      <hr class="my-4 border-zinc-900" />
      <!-- svelte-ignore a11y-autofocus -->
      <div class="flex justify-between">
        <button
          class="rounded-md border-2 border-zinc-900 px-1 text-sm font-semibold hover:bg-zinc-900 hover:text-white"
          on:click={() => dialog.close()}>cancel</button
        >
        <button
          class="rounded-md border-2 border-red-500 px-1 text-sm font-semibold text-red-500 hover:bg-red-500 hover:text-white"
          on:click={() => deleteItem()}>delete</button
        >
      </div>
    </div>
  </dialog>
</div>
