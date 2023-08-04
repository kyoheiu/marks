<script lang="ts">
  import { Toaster } from "svelte-french-toast";
  import { save } from "./Upload";

  export let edited: boolean;
  export let content: string;
</script>

<Toaster />
<div class="relative flex items-start">
  <button
    class="box-border h-6 w-12 rounded-full bg-sky-500 px-2 text-sm text-white hover:bg-sky-600"
    on:click={() => save(content)}
    title="save"><i class="ri-file-upload-line" title="save" /></button
  >
  {#if edited}
    <i
      class="ri-checkbox-blank-circle-fill absolute right-0 text-xs text-red-400"
    />
  {/if}
</div>
