<script lang="ts">
  import { onMount } from 'svelte'
  import { listNodes, importFile, exportFile } from '../tauri'
  export let currentVault: string
  let nodes = [] as Array<{id:string,parent?:string,name:string}>
  let selected = null as null | {id:string,name:string}
  let error = ''

  async function refresh() {
    try {
      nodes = await listNodes()
    } catch (e) { error = e.toString() }
  }

  function onSelect(n) { selected = n }

  async function doImport(e) {
    const input = document.createElement('input')
    input.type = 'file'
    input.onchange = async () => {
      if (input.files && input.files.length > 0) {
        const file = input.files[0]
        // Tauri can accept a file path string; for dev use absolute path from user
        // We'll call importFile with file.path (not available in browser); prompt user to input path instead
        const path = prompt('Enter absolute path to file to import (CLI fallback for now)')
        if (path) {
          await importFile(path, null)
          await refresh()
        }
      }
    }
    input.click()
  }

  async function doExport() {
    if (!selected) { alert('Select a file'); return }
    const dest = prompt('Enter destination directory')
    if (!dest) return
    await exportFile(selected.id, dest)
    alert('Export completed')
  }

  onMount(() => { refresh() })
</script>

<div style="background:var(--panel); padding:1rem; border-radius:6px">
  <div style="display:flex; justify-content:space-between; align-items:center">
    <h3>Explorer</h3>
    <div>
      <button class="button" on:click={doImport}>Import</button>
      <button class="button" on:click={refresh}>Refresh</button>
      <button class="button" on:click={doExport}>Export</button>
    </div>
  </div>
  {#if error}<div style="color:#ff6666">{error}</div>{/if}
  <div style="margin-top:1rem">
    {#each nodes as n}
      <div on:click={() => onSelect(n)} style="padding:0.4rem; cursor:pointer; background:{selected && selected.id===n.id ? '#071829' : 'transparent'}; border-radius:4px">
        <div style="font-weight:600">{n.name}</div>
        <div style="font-size:0.85rem; color:var(--muted)">id: {n.id}</div>
      </div>
    {/each}
  </div>
</div>
