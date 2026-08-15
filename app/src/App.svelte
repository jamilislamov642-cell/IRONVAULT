<script lang="ts">
  import Unlock from './components/Unlock.svelte'
  import Explorer from './components/Explorer.svelte'
  import ActivityLog from './components/ActivityLog.svelte'

  let unlocked = false
  let currentVault = ''

  function handleUnlocked(event) {
    unlocked = true
    currentVault = event.detail.vault
  }
</script>

<main>
  <div class="header">
    <h1>IRONVAULT</h1>
    <div style="margin-left:auto">
      {#if unlocked}
        <button class="button" on:click={() => { unlocked = false; currentVault = ''; }}>Lock</button>
      {/if}
    </div>
  </div>

  {#if !unlocked}
    <Unlock on:unlocked={handleUnlocked} />
  {:else}
    <div class="layout">
      <Explorer {currentVault} />
      <ActivityLog />
    </div>
  {/if}
</main>

<style>
  .layout { display:flex; gap:1rem; margin-top:1rem }
  Explorer, ActivityLog { flex:1 }
</style>
