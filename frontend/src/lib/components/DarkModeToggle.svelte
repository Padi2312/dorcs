<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { darkModeStore } from "../darkModeStore";

  onMount(() => {
    darkModeStore.subscribe((value) => {
      setDOMTheme(value);
    });

    const theme = localStorage.getItem("theme");
    darkModeStore.set(theme === "dark");
  });

  const handleDarkModeToggle = () => {
    localStorage.setItem("theme", !$darkModeStore ? "dark" : "light");
    darkModeStore.set(!$darkModeStore);
  };

  const setDOMTheme = (dark: boolean) => {
    if (dark) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  };
</script>

<button class="relative dark-mode-toggle" on:click={handleDarkModeToggle}>
  <div class="flex justify-center items-center h-full">
    {#if $darkModeStore}
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
