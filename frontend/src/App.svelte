<script lang="ts">
  import { onMount } from "svelte";
  import { ContentLoader } from "./lib/ContentLoader";
  import { Router } from "./lib/Router";
  import { DorcsSocket } from "./lib/Websocket";
  import Content from "./lib/components/Content.svelte";
  import Navbar from "./lib/components/Navbar.svelte";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import SyntaxHighlighter from "./lib/components/SyntaxHighlighter.svelte";
  import { fetchPageSettings, pageSettings } from "./lib/settingsStore";
  import type { Route } from "./lib/types";

  const router: Router = new Router();
  const contentLoader = new ContentLoader();
  let webSocket: DorcsSocket | null = null;

  let routes: Route[] = [];
  let content: string = "";
  let sidebarVisible = true;

  onMount(() => {
    fetchPageSettings();
    init();
    sidebarVisible = !checkMobileDevice();
    window.addEventListener("click", handleClick);
    window.addEventListener("popstate", onBackNavigation);

    // Subscribe pageSettings to changes
    pageSettings.subscribe((value) => {
      console.log(value);
      if (value?.dev) {
        webSocket = new DorcsSocket(
          `ws://${window.location.host}/ws`,
          onWebsocketMessage
        );
      } else {
        webSocket = null;
      }
    });

    return () => {
      window.removeEventListener("click", handleClick);
      window.removeEventListener("popstate", onBackNavigation);
    };
  });

  const onWebsocketMessage = async (wsData: MessageEvent) => {
    onRouteChange(
      { url: wsData.data, children: [], position: 0, title: "" },
      true
    );
  };

  const onBackNavigation = (ev: PopStateEvent) => {
    router.navigate(window.location.pathname);
  };

  const checkMobileDevice = () => {
    return window.innerWidth < 768;
  };

  const init = async () => {
    await router.init(onRouteChange);
    routes = router.getRoutes();
    if (window.location.pathname === "/") {
      window.history.pushState({}, "", "/index");
    }
    router.navigate(window.location.pathname);
  };

  const onRouteChange = async (route: Route, force = false) => {
    try {
      const data = await contentLoader.fetchContent(route.url, force);
      if (data) {
        content = data;
      }
    } catch (error) {
      console.error("Failed to fetch content:", error);
    }
  };

  const handleClick = async (event: Event) => {
    //check if element of tag <a> is clicked
    if (!(event.target as Element)?.matches("a")) {
      return;
    }

    const element = event.target as HTMLLinkElement;
    if (!element) {
      return;
    }
    if (element.href.startsWith(window.location.origin)) {
      event.preventDefault();
      router.navigate(element.href);
      if (checkMobileDevice()) {
        sidebarVisible = false;
      }
    }
  };

  function handleSidebarToggle() {
    sidebarVisible = !sidebarVisible;
  }
</script>

<!-- This svelte component is used for setting the syntax highlighting theme -->
<SyntaxHighlighter />

<div class="text-textColor bg-background h-full">
  <Navbar on:sidebarToggle={handleSidebarToggle} />
  <div class="flex bg-inherit">
    <Sidebar {routes} bind:visible={sidebarVisible} />
    <div class="w-full flex justify-center max-w-full">
      <Content {content} />
    </div>
  </div>
</div>
