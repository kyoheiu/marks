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
    class="w-7/8 max-w-sm drop-shadow-2xl"
    bind:this={dialog}
    on:close={() => (showModal = false)}
    on:click|self={() => dialog.close()}
  >
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div on:click|stopPropagation>
      <div class="mb-2 break-all p-2">
        Are you sure to delete <span class="underline">{item}</span>?<br />
        This action is irreversible.
      </div>
      <hr class="my-4 text-border" />
      <!-- svelte-ignore a11y-autofocus -->
      <div class="flex justify-between">
        <button class="px-1 text-sm" on:click={() => dialog.close()}
          >cancel</button
        >
        <button
          class="rounded bg-warning px-2 py-1 text-sm text-item_background"
          on:click={() => deleteItem()}>delete</button
        >
      </div>
    </div>
  </dialog>
</div>
