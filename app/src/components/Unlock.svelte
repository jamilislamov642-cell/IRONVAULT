<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { createVault, unlockVault } from '../tauri'

  const dispatch = createEventDispatcher()
  let path = ''
  let password = ''
  let confirm = ''
  let mode: 'create' | 'unlock' = 'unlock'
  let busy = false
  let error = ''

  async function doCreate() {
    try {
      if (password !== confirm) { error = 'Passwords do not match'; return }
      busy = true
      await createVault(path, password)
      dispatch('unlocked', { vault: path })
    } catch (e) { error = e.toString() }
    finally { busy = false }
  }

  async function doUnlock() {
    try {
      busy = true
      await unlockVault(path, password)
      dispatch('unlocked', { vault: path })
    } catch (e) { error = e.toString() }
    finally { busy = false }
  }
</script>

<section>
  <div style="display:flex; gap:1rem; align-items:center">
    <label><input type="radio" bind:group={mode} value="unlock"> Unlock</label>
    <label><input type="radio" bind:group={mode} value="create"> Create</label>
  </div>
  <div style="margin-top:1rem; max-width:40rem">
    <input placeholder="Vault path" bind:value={path} style="width:100%" />
    <input placeholder="Password" type="password" bind:value={password} style="width:100%; margin-top:0.5rem" />
    {#if mode === 'create'}
      <input placeholder="Confirm password" type="password" bind:value={confirm} style="width:100%; margin-top:0.5rem" />
    {/if}
    <div style="margin-top:0.5rem">
      {#if mode === 'create'}
        <button class="button" on:click={doCreate} disabled={busy}>Create and Unlock</button>
      {:else}
        <button class="button" on:click={doUnlock} disabled={busy}>Unlock</button>
      {/if}
    </div>
    {#if error}<div style="color:#ff6666;margin-top:0.5rem">{error}</div>{/if}
  </div>
</section>
