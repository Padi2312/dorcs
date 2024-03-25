<script lang="ts">
  import type { Route } from "../types";
  import IconChevronRight from "./icons/IconChevronRight.svelte";
  import IconChevronDown from "./icons/IconChevronDown.svelte";

  export let route: Route;
  let open = false;

  function toggleOpen(event: Event) {
    event.stopPropagation();
    open = !open;
  }
</script>

<div class="menu-section">
  <div class="menu-section-title">
    {#if route.url}
      <a class="flex-1" href={route.url}>
        {route.title}
      </a>
    {:else}
      <button class="flex-1 text-start" on:click={toggleOpen}
        >{route.title}</button
      >
    {/if}

    {#if route.children}
      <button class="hover:bg-gray-300 rounded-md p-1" on:click={toggleOpen}>
        {#if open}
          <IconChevronDown />
        {:else}
          <IconChevronRight />
        {/if}
      </button>
    {/if}
  </div>

  {#if open && route.children}
    <div class="submenu">
      {#each route.children as child}
        {#if child.is_section}
          <svelte:self route={child} />
        {:else}
          <a class="menu-section-title" href={child.url} on:click>
            {child.title}
          </a>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style>
  .menu-section {
    @apply pl-2 py-2  block rounded-lg;
  }

  .menu-section-title {
    @apply p-2 rounded-lg flex justify-between space-x-4 font-bold;
  }

  .menu-section-title:hover {
    @apply bg-gray-200;
  }

  .submenu {
    @apply list-none pl-5;
  }
</style>
