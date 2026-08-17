<script lang="ts">
  let logs = [
    { ts: Date.now(), event: 'Vault unlocked' },
  ]
</script>

<div style="background:var(--panel); padding:1rem; border-radius:6px">
  <h3>Activity</h3>
  <div style="margin-top:0.5rem">
    {#each logs as l}
      <div style="padding:0.25rem 0">{new Date(l.ts).toLocaleString()} — {l.event}</div>
    {/each}
  </div>
</div>
