<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { pageSettings } from "../settingsStore";
  import DarkModeToggle from "./DarkModeToggle.svelte";

  const dispatch = createEventDispatcher();

  function handleSidebarToggle() {
    dispatch("sidebarToggle");
  }
</script>

<div class="navbar relative">
  <div class="flex">
    <button
      class="sidebar-toggle md:hidden sm:block me-4"
      on:click={handleSidebarToggle}
    >
      ☰
    </button>
    {#if $pageSettings?.icon}
      <img src={$pageSettings?.icon} alt="logo" class="navbar-icon me-2" />
    {/if}
    <div class="navbar-title">{$pageSettings?.page_title}</div>
  </div>
  <DarkModeToggle />
</div>

<style>
  .navbar {
    @apply px-6 w-full h-12 relative  z-10  items-center font-bold text-xl mb-1 
    flex justify-between shadow-md;
  }

  .navbar-icon {
    @apply h-8 w-8;
  }
</style>
