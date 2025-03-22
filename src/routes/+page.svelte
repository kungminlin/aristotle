<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  import { Input } from "$lib/components/ui/input"
  import { Button } from "$lib/components/ui/button"

  let title = $state("");

  async function handleUpdateFile(e: Event) {
    e.preventDefault();
    await invoke("update_file", { path: "/test.md", blob: title });
  }
</script>

<main class="container">
  <div class="flex flex-col gap-2 p-4">
    <Input placeholder="Title" bind:value={title} />
    <Button onclick={handleUpdateFile}>
      Submit
    </Button>
  </div>
</main>
