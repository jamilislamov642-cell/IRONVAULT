<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  export let nodeId: string
  let content = ''
  let error = ''

  async function loadPreview() {
    try {
      // Call backend helper to produce a safe text preview (core will need to support this)
      // For now, attempt to export a temp copy via backend and read small amount
      // We'll call an IPC 'preview_node' that returns text preview or metadata
      // @ts-ignore
      const res = await invoke('preview_node', { node_id: nodeId })
      // @ts-ignore
      content = res?.preview || '<no preview available>'
    } catch (e) { error = e.toString() }
  }

  onMount(() => { loadPreview() })
</script>

<div style="background:rgba(255,255,255,0.03); padding:1rem; border-radius:6px; height:100%">
  {#if error}
    <div style="color:#ff6666">{error}</div>
  {:else}
    <pre style="white-space:pre-wrap; color:var(--muted)">{content}</pre>
  {/if}
</div>
