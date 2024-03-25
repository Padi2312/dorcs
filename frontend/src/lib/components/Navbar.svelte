<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";

  let darkMode = false;
  const dispatch = createEventDispatcher();

  function handleSidebarToggle() {
    dispatch("sidebarToggle");
  }

  function handleDarkModeToggle() {
    darkMode = !darkMode;
    localStorage.setItem("theme", darkMode ? "dark" : "light");
    darkMode
      ? document.documentElement.classList.add("dark")
      : document.documentElement.classList.remove("dark");
  }

  onMount(() => {
    const theme = localStorage.getItem("theme");
    if (theme === "dark") {
      darkMode = true;
      document.documentElement.classList.add("dark");
    }
  });
</script>

<div class="navbar">
  <button
    class="sidebar-toggle md:hidden sm:block"
    on:click={handleSidebarToggle}
  >
    ☰
  </button>
  <div class="navbar-title">Documentation</div>
  <button class="dark-mode-toggle" on:click={handleDarkModeToggle}
    >{darkMode ? "🌞" : "🌛"}</button
  >
</div>

<style>
  .navbar {
    @apply px-6 w-full h-12 relative  z-10  items-center font-bold text-xl mb-1 
    flex justify-between shadow-md;
  }
</style>
