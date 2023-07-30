<script lang="ts">
  import "remixicon/fonts/remixicon.css";
  import { onMount } from "svelte";
  import DialogToDelete from "./lib/DialogToDelete.svelte";
  import Upload from "./lib/Upload.svelte";
  import Footer from "./lib/Footer.svelte";

  enum State {
    Top,
    Content,
    New,
  }

  interface ResItem {
    name: string;
    modified: number;
  }

  interface Item {
    name: string;
    modified: number;
    showModal: boolean;
  }

  let state = State.Top;
  let fileName: string;
  let filter: string | null;
  let query: string;
  let queryFixed: string;
  let items: Item[];
  let seeMore = false;
  let content: string;
  let newName: string;

  onMount(async () => {
    const res = await fetch("/item");
    const j: ResItem[] = await res.json();
    items = j.map((x) => {
      return { ...x, showModal: false };
    });
  });

  // for test
  // onMount(() => {
  //   items = [
  //     { name: "test.md", showModal: false },
  //     { name: "test2.md", showModal: false },
  //     { name: "loooooooooooooooooooooong.md", showModal: false },
  //   ];
  // });

  const readItem = async (item: string) => {
    const res = await fetch(`/item?item=${item}`);
    const body = await res.text();
    content = body;
    fileName = item;
    newName = item;
    state = State.Content;
  };

  const search = async () => {
    const res = await fetch("/api/search", {
      method: "POST",
      body: query,
    });
    if (res.ok) {
      const j: ResItem[] = await res.json();
      console.log(j);
      items = j.map((x) => {
        return {
          ...x,
          showModal: false,
        };
      });
      filter = undefined;
      queryFixed = query;
      state = State.Top;
    }
  };

  const rename = async () => {
    const _res = await fetch("/api/rename", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        original: fileName,
        new: newName
      })
    });
  }
</script>

<main class="flex h-screen flex-col items-center">
  <div class="flex-grow">
    <div class="my-4 flex items-center justify-center">
      <a href="/"><div class="">marks</div></a>
      &nbsp; &nbsp;
      <button
        on:click={() => {
          fileName = undefined;
          content = undefined;
          state = State.New;
        }}><i class="ri-add-circle-line" /></button
      >
      &nbsp; &nbsp;
      <input
        class="rounded-full border p-1 text-sm"
        bind:value={query}
        placeholder="search"
        on:keydown={(e) => e.key === "Enter" && search()}
      />
    </div>
    {#if state === State.Top}
      <div class="flex flex-col items-center">
        {#if !queryFixed}
          <input
            class="mb-2 mt-4 rounded-full border p-1 text-sm"
            bind:value={filter}
            placeholder="filter"
          />
        {/if}
        {#if queryFixed}
          <div class="m-2">Query: {queryFixed}</div>
        {/if}
      </div>
      <div class="flex flex-col items-center">
        {#if items}
          {#each filter ? items.filter( (x) => x.name.includes(filter) ) : seeMore ? items : items.slice(0, 5) as item}
            <div
              class="w-40 my-2 flex items-center justify-between border-b border-zinc-800"
            >
              <button
                class="line-clamp-1 font-mono text-sm"
                on:click={() => !item.showModal && readItem(item.name)}
                >{item.name}</button
              >
              &nbsp;
              <button on:click={() => (item.showModal = true)}
                ><i
                  class="ri-delete-bin-2-line text-sm text-zinc-600"
                /></button
              >
            </div>
            <DialogToDelete bind:showModal={item.showModal} {item} />
          {/each}
        {/if}
        {#if !filter && !queryFixed && !seeMore}
          <button class="m-2" on:click={() => (seeMore = true)}
            ><i class="ri-more-2-line" /></button
          >
        {/if}
      </div>
    {:else if state === State.Content}
      <form on:submit={() => rename()}>
        <input class="mb-2 w-72 sm:w-120 md:w-144 p-2 font-mono text-sm" bind:value={newName} />
      </form>
      <textarea
        class="h-3/4 w-72 sm:w-120 md:w-144 border p-2 font-mono text-sm text-zinc-900"
        rows="25"
        bind:value={content}
      />
      <Upload {fileName} {content} />
    {:else}
    <div class="flex flex-col">
      <input
        class="mb-2 bg-zinc-200 font-mono text-sm"
        bind:value={fileName}
        placeholder="file name"
      />
      <textarea
        class="h-3/4 w-72 sm:w-120 md:w-144 border p-2 font-mono text-sm text-zinc-900"
        rows="25"
        bind:value={content}
        placeholder="content"
      />
      <Upload {fileName} {content} />
      </div>
    {/if}
  </div>
  <Footer />
</main>
