*** Begin Patch
*** Update File: app/src/App.svelte
@@
   import ActivityLog from './components/ActivityLog.svelte'
+  import CommandPalette from './components/CommandPalette.svelte'
@@
   {#if !unlocked}
     <Unlock on:unlocked={handleUnlocked} />
   {:else}
     <div class="layout">
       <Explorer {currentVault} />
       <ActivityLog />
     </div>
   {/if}
+  <CommandPalette />
 </main>
*** End Patch
