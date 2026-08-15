*** Begin Patch
*** Add File: app/src/App.svelte
+<script lang="ts">
+  let unlocked = false;
+  let vaultPath = '';
+  let password = '';
+  function createVault() {
+    alert('Create vault flow will be implemented via Tauri IPC');
+  }
+  function unlock() {
+    alert('Unlock flow will be implemented via Tauri IPC');
+  }
+</script>
+
+<main>
+  <h1>IRONVAULT</h1>
+  <section>
+    <h2>Create / Unlock</h2>
+    <input placeholder="vault path" bind:value={vaultPath} />
+    <input placeholder="password" type="password" bind:value={password} />
+    <button on:click={createVault}>Create Vault</button>
+    <button on:click={unlock}>Unlock Vault</button>
+  </section>
+  <section>
+    <h2>Explorer</h2>
+    <p>Explorer and command palette will appear here after unlock.</p>
+  </section>
+</main>
+
+<style>
+  main { color: #e6eef6; background: #0f1720; padding: 2rem; font-family: system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial; }
+  h1 { font-size: 1.5rem; }
+  section { margin-top: 1rem; }
+  input { display: block; margin: 0.5rem 0; padding: 0.5rem; width: 100%; max-width: 40rem; }
+  button { margin-right: 0.5rem; padding: 0.5rem 1rem; background: #0b7285; color: white; border: none; border-radius: 4px; }
+</style>
+
*** End Patch
