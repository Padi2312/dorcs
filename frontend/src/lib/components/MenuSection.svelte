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
    {#if route.url.length !== 0}
      <a class="flex-1" href={route.url.replace("pages/", "/")}>
        {route.title}
      </a>
    {:else}
      <button class="flex-1 text-start" on:click={toggleOpen}
        >{route.title}</button
      >
    {/if}

    {#if route.children?.length !== 0}
      <button class="hover:bg-gray-300 rounded-md" on:click={toggleOpen}>
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
        {#if child.children?.length !== 0}
          <svelte:self route={child} />
        {:else}
          <a
            class="menu-section-title my-1"
            href={child.url.replace("pages/", "/")}
            on:click
          >
            {child.title}
          </a>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style>
  .menu-section {
    @apply py-2  block rounded-lg;
  }

  .menu-section a {
    @apply text-textColor;
  }

  .menu-section-title {
    @apply p-2 rounded-lg flex justify-between space-x-4 font-bold items-center;
  }

  .menu-section-title:hover {
    @apply bg-zinc-200 dark:bg-zinc-600;
  }

  .submenu {
    @apply list-none pl-5;
  }
</style>
