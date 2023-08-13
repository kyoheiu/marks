<script lang="ts">
  import { DEFAULT_LIST_NUMBER, readItem, state } from "./stores";
  import DialogToDelete from "./DialogToDelete.svelte";
  import moment from "moment";

  const toDuration = (time: number) => {
    return moment.unix(time).fromNow();
  };
</script>

<ul class="flex flex-col items-center p-2">
  {#each $state.filter ? $state.items.filter( (x) => x.name.includes($state.filter) ) : $state.seeMore ? $state.items : $state.items.slice(0, DEFAULT_LIST_NUMBER) as item}
    <li class="list-none">
      <div class="mb-4 w-64 list-none rounded bg-item_background p-4 sm:w-96">
        <div class="mb-2 max-w-full truncate text-lg font-bold">
          {item.name}
        </div>
        <div class="line-clamp-2 h-10 text-sm">
          {#if item.desc}
            {item.desc}
          {:else}
            <i>No contents.</i>
          {/if}
        </div>
        <hr class="my-4 text-border" />
        <div class="flex items-baseline text-sm">
          <span class="text-xs">{toDuration(item.modified)}</span>
          <button
            class="ml-auto text-xs text-subtle"
            on:click={() => (item.showModal = true)}
            title="delete"
          >
            Delete
          </button>
          &nbsp; &nbsp;
          <button
            class="rounded bg-base px-2 py-1 text-item_background"
            on:click={() => !item.showModal && readItem(item.name)}>View</button
          >
        </div>
        <DialogToDelete bind:showModal={item.showModal} item={item.name} />
      </div>
    </li>
  {/each}
</ul>
