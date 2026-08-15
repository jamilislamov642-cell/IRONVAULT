<script lang="ts">
  import { onMount } from 'svelte'
  let open = false
  let query = ''
  let results = [] as Array<{id:string, name:string}>

  function toggle() { open = !open }

  onMount(() => {
    window.addEventListener('keydown', (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault(); toggle()
      }
    })
  })
</script>

{#if open}
  <div style="position:fixed; left:50%; top:20%; transform:translateX(-50%); width:40rem; background:var(--panel); padding:1rem; border-radius:6px; box-shadow:0 8px 24px rgba(0,0,0,0.6)">
    <input placeholder="Type a command or search..." bind:value={query} style="width:100%; padding:0.5rem" />
    <div style="margin-top:0.5rem; max-height:20rem; overflow:auto">
      <div style="color:var(--muted)">Commands (type to filter):</div>
      <div style="margin-top:0.5rem">
        <div>Open Vault</div>
        <div>Lock Vault</div>
        <div>Import Files</div>
        <div>Export</div>
        <div>Search</div>
        <div>Settings</div>
      </div>
    </div>
  </div>
{/if}
