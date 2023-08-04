<script lang="ts">
  import { Toaster } from "svelte-french-toast";
  import { save } from "./Upload";

  export let content;
</script>

<Toaster />
<button
  class="box-border w-12 h-6 rounded-full bg-sky-500 px-2 text-sm text-white"
  on:click={() => save(content)} title="save"
  ><i class="ri-file-upload-line" title="save" /></button
>
