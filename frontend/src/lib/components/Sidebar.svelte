<script lang="ts">
  import { slide } from "svelte/transition";
  import type { Route } from "../types";
  import MenuSection from "./MenuSection.svelte";

  export let visible: boolean = true;
  export let routes: Route[] = [];
</script>

{#if visible}
  <aside
    class="sidebar"
    class:visible
    transition:slide={{ duration: 350, axis: "x" }}
  >
    {#each routes as route}
      <MenuSection {route} />
    {/each}
  </aside>
{/if}

<style>
  .sidebar {
    @apply px-4 lg:w-96 md:w-72 h-full border-r dark:border-gray-700 border-gray-200 z-30
    hidden w-full sm:w-screen md:block md:relative absolute bg-background;
  }

  .sidebar.visible {
    @apply block;
  }
</style>
