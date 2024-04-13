<script lang="ts">
  import "highlightjs-copy/dist/highlightjs-copy.min.css";
  import { onMount } from "svelte";
  import { darkModeStore } from "../darkModeStore";

  const lightThemeUrl =
    "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github.min.css";
  const darkThemeUrl =
    "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark-dimmed.min.css";

  onMount(() => {
    darkModeStore.subscribe((value) => {
      setHighlightTheme(value);
    });
  });

  const setHighlightTheme = (dark: boolean) => {
    let url = dark ? darkThemeUrl : lightThemeUrl;
    const link = document.createElement("link");
    link.rel = "stylesheet";
    link.href = url;

    // Remove old theme if exists
    const oldLink = document.querySelector(
      "link[href^='https://cdnjs.cloudflare.com/ajax/libs/highlight.js/']"
    );
    if (oldLink) {
      document.head.removeChild(oldLink);
    }

    document.head.appendChild(link);
  };
</script>
