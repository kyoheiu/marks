<script lang="ts">
  import "remixicon/fonts/remixicon.css";
  import { onMount } from "svelte";
  import { Page, state, resetItem } from "./lib/stores";
  import Footer from "./lib/Footer.svelte";
  import Header from "./lib/Header.svelte";
  import Top from "./lib/Top.svelte";
  import View from "./lib/View.svelte";
  import Edit from "./lib/Edit.svelte";

  if (history.scrollRestoration) {
    history.scrollRestoration = "manual";
  }

  onMount(async () => {
    resetItem();
  });
</script>

<main class="flex min-h-screen flex-col items-center">
  <Header />
  <div class="grow">
    {#if $state.page === Page.Top}
      <Top />
    {:else if $state.page === Page.View}
      <View />
    {:else if $state.page === Page.Edit}
      <Edit />
    {/if}
  </div>
</main>
