<script>
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  let darkMode = false;
  onMount(() => {
    const theme = localStorage.getItem("theme");
    if (theme === "dark") {
      darkMode = true;
      document.documentElement.classList.add("dark");
    }
  });
  function handleDarkModeToggle() {
    darkMode = !darkMode;
    localStorage.setItem("theme", darkMode ? "dark" : "light");
    darkMode
      ? document.documentElement.classList.add("dark")
      : document.documentElement.classList.remove("dark");
  }
</script>

<button class="relative dark-mode-toggle" on:click={handleDarkModeToggle}>
  <div class="flex justify-center items-center h-full">
    {#if darkMode}
      <span
        in:fade={{ duration: 200 }}
        out:fade={{ duration: 200 }}
        class="absolute"
      >
        🌞
      </span>
    {:else}
      <span
        in:fade={{ duration: 200 }}
        out:fade={{ duration: 200 }}
        class="absolute"
      >
        🌛
      </span>
    {/if}
  </div>
</button>
