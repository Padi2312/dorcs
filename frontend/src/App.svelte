<script lang="ts">
  import { onMount } from "svelte";
  import { ContentLoader } from "./lib/ContentLoader";
  import { Router } from "./lib/Router";
  import Content from "./lib/components/Content.svelte";
  import Navbar from "./lib/components/Navbar.svelte";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import type { Route } from "./lib/types";
  const router: Router = new Router();
  const contentLoader = new ContentLoader();

  let routes: Route[] = [];
  let content: string = "";
  let sidebarVisible = true;

  onMount(() => {
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
    // console.log(ev);
    // ev.preventDefault();
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
    console.log("Sidebar toggle", sidebarVisible);
  }
</script>

<div class="text-gray-700 dark:text-gray-200 bg-white dark:bg-zinc-800 h-full">
  <Navbar on:sidebarToggle={handleSidebarToggle} />
  <div class="flex">
    <Sidebar {routes} bind:visible={sidebarVisible} />
    <div class="w-full flex justify-center relative max-w-full">
      <Content {content} />
    </div>
  </div>
</div>
