<script lang="ts">
  import 'remixicon/fonts/remixicon.css'
  import { onMount } from "svelte";
  import DialogToDelete from "./lib/DialogToDelete.svelte";

  enum State {
    Top,
    Content,
    New,
  }

  interface Item {
    name: string;
    showModal: boolean;
  }

  interface Payload {
    name: string;
    body: string;
  }

  interface Res {
    result: string[];
  }

  let state = State.Top;
  let fileName: string;
  let filter: string | null;
  let query: string;
  let items: Item[];
  let content: string;
  let showDialog = false;

  onMount(async () => {
    const res = await fetch("/item");
    const j: string[] = await res.json();
    items = j.slice(1).map((x) => {return {name: x, showModal: false}});
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
    state = State.Content;
  };

  const save = async (payload: Payload) => {
    const _res = await fetch(`/item`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });
  };

  const search = async () => {
    const res = await fetch("/api/search", {
      method: "POST",
      body: query,
    });
    if (res.ok) {
      const j: Res = await res.json();
      items = j.result.map((x) => {
        return {
        name: x,
        showModal: false,
      }
    });
      state = State.Top;
    }
  };
</script>

<main class="flex flex-col items-center">
  <div class="max-w-5/6 my-4 flex items-center justify-between">
    <a href="/"><div class="">marks</div></a>
    <button
      on:click={() => {
        state = State.New;
      }}><i class="ri-add-circle-line" /></button
    >
    <input
      class="w-1/2 rounded-full border p-1"
      bind:value={query}
      placeholder="search"
      on:keydown={(e) => e.key === "Enter" && search()}
    />
  </div>
  {#if state === State.Top}
    <input
      class="mb-2 mt-4 w-3/4 rounded-full border p-1"
      bind:value={filter}
      placeholder="filter"
    />
    <div class="w=5/6 flex flex-col items-start">
      {#if items}
        {#each filter ? items.filter( (x) => x.name.includes(filter) ) : items as item}
          <div
            class="my-2 flex w-full max-w-sm justify-between border-b border-zinc-800"
          >
            <button class="line-clamp-1 w-40" on:click={() => readItem(item.name)}
              >{item.name}</button
            >
            <button on:click={() => (item.showModal = true)}
              ><i class="ri-delete-bin-2-line" /></button
            >
            <DialogToDelete showModal={item.showModal} {item} />
          </div>
        {/each}
      {/if}
    </div>
  {:else if state === State.Content}
    <div>{fileName}</div>
    <textarea
      class="h-3/4 w-full border font-mono"
      rows="20"
      bind:value={content}
    />
    <button on:click={() => save({ name: fileName, body: content })}
      ><i class="ri-file-upload-line" /></button
    >
  {:else}
    <input
      class="mb-2 w-full bg-zinc-200 font-mono"
      bind:value={fileName}
      placeholder="file name"
    />
    <textarea
      class="h-3/4 w-full border font-mono"
      rows="20"
      bind:value={content}
      placeholder="content"
    />
    <button class="m-2" on:click={() => save({ name: fileName, body: content })}
      ><i class="ri-file-upload-line" /></button
    >
  {/if}
</main>
