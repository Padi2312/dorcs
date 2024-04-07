<script lang="ts">
  import { onMount } from "svelte";
  import { ContentLoader } from "./lib/ContentLoader";
  import { Router } from "./lib/Router";
  import Content from "./lib/components/Content.svelte";
  import Navbar from "./lib/components/Navbar.svelte";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import type { Route } from "./lib/types";
  import { fetchPageSettings } from "./lib/settingsStore";

  const router: Router = new Router();
  const contentLoader = new ContentLoader();

  let routes: Route[] = [];
  let content: string = "";
  let sidebarVisible = true;

  onMount(() => {
    fetchPageSettings();
    init();
    sidebarVisible = !checkMobileDevice();
    window.addEventListener("click", handleClick);
    window.addEventListener("popstate", onBackNavigation);

    return () => {
      window.removeEventListener("click", handleClick);
      window.removeEventListener("popstate", onBackNavigation);
    };
  });

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

  const onRouteChange = async (route: Route) => {
    try {
      const data = await contentLoader.fetchContent(route.url);
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

<div class="text-textColor bg-background h-full">
  <Navbar on:sidebarToggle={handleSidebarToggle} />
  <div class="flex bg-inherit">
    <Sidebar {routes} bind:visible={sidebarVisible} />
    <div class="w-full flex justify-center max-w-full">
      <Content {content} />
    </div>
  </div>
</div>
