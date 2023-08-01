<script>
  export let showModal; // boolean
  export let item; // string

  let dialog; // HTMLDialogElement

  const deleteItem = async () => {
    const _res = await fetch("/item", {
      method: "DELETE",
      body: item.name,
    });
    dialog.close();
    window.location.assign("/");
  };

  $: if (dialog && showModal) dialog.showModal();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<div class="m-auto">
  <dialog
    class="w-5/6 max-w-sm rounded-md bg-slate-100"
    bind:this={dialog}
    on:close={() => (showModal = false)}
    on:click|self={() => dialog.close()}
  >
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div on:click|stopPropagation>
      <div class="mb-4 break-words p-2 text-xl text-neutral-800">
        Are you sure to delete <code class="rounded-md bg-sky-100 p-1"
          >{item.name}</code
        >?<br />
        This action is irreversible.
      </div>
      <hr class="my-4 border-neutral-800" />
      <!-- svelte-ignore a11y-autofocus -->
      <div class="flex justify-between">
        <button
          class="rounded-md border-2 border-neutral-800 px-1 text-sm font-semibold hover:bg-neutral-800 hover:text-slate-100"
          on:click={() => dialog.close()}>cancel</button
        >
        <button
          class="rounded-md border-2 border-rose-500 px-1 text-sm font-semibold text-rose-500 hover:bg-rose-500 hover:text-slate-50"
          on:click={() => deleteItem()}>delete</button
        >
      </div>
    </div>
  </dialog>
</div>
