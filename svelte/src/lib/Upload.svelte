<script lang="ts">
  import { Toaster } from "svelte-french-toast";
  import { save } from "./Upload";

  export let content;
</script>

<Toaster />
<button
  class="col-span-2 box-border h-6 rounded-full bg-sky-500 px-2 text-sm text-white"
  on:click={() => save(content)}
  ><i class="ri-file-upload-line" title="save" /></button
>
