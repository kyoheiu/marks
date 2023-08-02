import toast from "svelte-french-toast";
import { state } from "./stores";
import { get } from "svelte/store";

export const save = async (content: string) => {
  const s = get(state);
  if (s.newName === "" || !s.newName) {
    toast.error("File name required.", {
      duration: 2000,
    });
    return;
  }
  const res = await fetch(`/item`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      original: s.fileName,
      new: s.newName,
      content: content,
    }),
  });

  if (!res.ok) {
    const message = await res.text();
    toast.error(`Error occured: ${message}`, {
      duration: 2000,
    });
    return;
  }

  state.update((s) => {
    return {
      ...s,
      fileName: s.newName,
      content: content,
    };
  });

  toast.success("Saved.", {
    duration: 2000,
  });
};
