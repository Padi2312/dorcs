<script lang="ts">
  export let content: string = "";
</script>

<main class="flex md:p-10 p-4">
  <div
    class="content-container lg:max-w-screen-lg md:max-w-screen-md sm:max-w-none"
  >
    {@html content}
  </div>
</main>

<style>
  .content-container {
    @apply h-full w-full;
  }
</style>
