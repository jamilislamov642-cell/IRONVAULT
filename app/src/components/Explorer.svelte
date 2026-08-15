<script lang="ts">
  import { onMount } from 'svelte'
  import { listNodes, importFile, exportFile } from '../tauri'
  import Preview from './Preview.svelte'
  export let currentVault: string
  let nodes = [] as Array<{id:string,parent?:string,name:string}>
  let selected = null as null | {id:string,name:string}
  let error = ''
  let importProgress = 0
  let importing = false

  async function refresh() {
    try {
      nodes = await listNodes()
    } catch (e) { error = e.toString() }
  }

  function onSelect(n) { selected = n }

  async function doImportPrompt() {
    const path = prompt('Enter absolute path to file to import')
    if (path) {
      await doImport(path)
    }
  }

  async function doImport(path: string) {
    try {
      importing = true
      importProgress = 0
      // emit progress events from backend; here we just call import
      const id = await importFile(path, null)
      alert(`Imported id ${id}`)
      await refresh()
    } catch(e) { error = e.toString() }
    finally { importing = false; importProgress = 0 }
  }

  async function doExport() {
    if (!selected) { alert('Select a file'); return }
    const dest = prompt('Enter destination directory')
    if (!dest) return
    await exportFile(selected.id, dest)
    alert('Export completed')
  }

  // Listen for import progress events from Tauri
  onMount(() => {
    // use window.__TAURI__ events if available
    try {
      // @ts-ignore
      window.__TAURI__.event.listen('import-progress', e => {
        // e.payload expected {progress: number}
        // @ts-ignore
        const p = e.payload?.progress || 0
        importProgress = p
      })
    } catch (e) {
      // ignore in non-tauri env
    }
    refresh()
  })
</script>

<div style="background:var(--panel); padding:1rem; border-radius:6px">
  <div style="display:flex; justify-content:space-between; align-items:center">
    <h3>Explorer</h3>
    <div>
      <button class="button" on:click={doImportPrompt} disabled={importing}>Import</button>
      <button class="button" on:click={refresh}>Refresh</button>
      <button class="button" on:click={doExport}>Export</button>
    </div>
  </div>
  {#if error}<div style="color:#ff6666">{error}</div>{/if}
  {#if importing}
    <div style="margin-top:0.5rem">
      Importing... {importProgress}%
      <progress max="100" value={importProgress}></progress>
    </div>
  {/if}
  <div style="margin-top:1rem; display:flex; gap:1rem">
    <div style="width:40%">
      {#each nodes as n}
        <div on:click={() => onSelect(n)} style="padding:0.4rem; cursor:pointer; background:{selected && selected.id===n.id ? '#071829' : 'transparent'}; border-radius:4px">
          <div style="font-weight:600">{n.name}</div>
          <div style="font-size:0.85rem; color:var(--muted)">id: {n.id}</div>
        </div>
      {/each}
    </div>
    <div style="flex:1">
      {#if selected}
        <Preview nodeId={selected.id} />
      {:else}
        <div style="color:var(--muted)">Select a file to preview</div>
      {/if}
    </div>
  </div>
</div>
