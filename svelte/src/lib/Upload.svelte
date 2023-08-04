<script lang="ts">
  import { Toaster } from "svelte-french-toast";
  import { save } from "./Upload";

  export let edited: boolean;
  export let content: string;
</script>

<Toaster />
<div class="relative flex items-start">
<button
  class="box-border w-12 h-6 rounded-full bg-sky-500 hover:bg-sky-600 px-2 text-sm text-white"
  on:click={() => save(content)} title="save"
  ><i class="ri-file-upload-line" title="save" /></button
>
{#if edited}
  <i class="absolute right-0 ri-checkbox-blank-circle-fill text-red-400 text-xs"></i>
{/if}
</div>