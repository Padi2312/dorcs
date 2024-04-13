<script lang="ts">
  import hljs from "highlight.js";
  import { afterUpdate } from "svelte";
  // @ts-ignore
  import CopyButtonPlugin from "highlightjs-copy";
  import "highlightjs-copy/dist/highlightjs-copy.min.css";

  export let content: string = "";

  // TODO: Handle this in a better way
  afterUpdate(async () => {
    if (content) {
      hljs.addPlugin(new CopyButtonPlugin());
      hljs.highlightAll();
    }
  });
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
