class ContentFetcher {
  constructor() {
    this.cache = new Map();
  }

  fetchContent(url) {
    if (!this.cache.has(url)) {
      const contentPromise = fetch(url).then((response) => response.text());
      this.cache.set(url, contentPromise);
    }
    return this.cache.get(url);
  }
}

class Router {
  constructor(contentFetcher) {
    this.routes = [];
    this.contentFetcher = contentFetcher;
  }

  async fetchRoutes() {
    try {
      const response = await fetch("/routes.json");
      const data = await response.json();
      this.routes = data; // Update the routes directly
    } catch (error) {
      console.error(error);
    }
  }

  navigate(url) {
    url = url === "/" ? "/index" : url;
    const route = this.findUrlInTree(this.routes, url);
    if (route) {
      this.contentFetcher
        .fetchContent(`/pages/${url}.html`)
        .then((template) => {
          document.querySelector(".content-container").innerHTML = template;
          this.changePageTitle(route.title);
        });
    }
  }

  findUrlInTree(tree, url) {
    if (!tree || tree.length === 0) return null;

    for (const node of tree) {
      if (node.url === url) return node;
      if (node.children) {
        const result = this.findUrlInTree(node.children, url);
        if (result) return result;
      }
    }
    return null;
  }

  generateSidebarNavigation() {
    const sidebar = document.querySelector(".sidebar");
    this.buildNavigation(this.routes, sidebar);
  }

  buildNavigation(routes, parentElement) {
    routes.forEach((route) => {
      if (route.is_section) {
        const section = this.createSection(route);
        parentElement.appendChild(section);
        if (route.children) {
          const submenu = document.createElement("div");
          submenu.className = "submenu";
          submenu.style.display = "none";
          section.appendChild(submenu);
          this.buildNavigation(route.children, submenu);
        }
      } else {
        const menuItem = this.createMenuItem(route);
        parentElement.appendChild(menuItem);
      }
    });
  }

  createSection(route) {
    const sectionDiv = document.createElement("div");
    sectionDiv.className = "menu-section";

    const sectionTitle = document.createElement(route.url ? "a" : "div");
    sectionTitle.className = "menu-section-title";
    sectionTitle.textContent = route.title;
    if (route.url) sectionTitle.href = route.url;

    sectionTitle.addEventListener("click", () => {
      this.toggleSubMenu(sectionTitle);
      // Only navigate if the section has a URL
      if (route.url) this.navigate(route.url);
    });
    // Arrow for toggling the submenu
    const arrowSpan = document.createElement("span");
    arrowSpan.className = "arrow";
    arrowSpan.innerHTML = "&#9660;";

    // Prevent redirect when arrow is clicked
    arrowSpan.addEventListener("click", (event) => {
      event.stopPropagation(); // Stop the click event from bubbling up
      this.toggleSubMenu(sectionTitle); // Toggle the submenu visibility
    });

    // Append the arrow span to the section title
    sectionTitle.appendChild(arrowSpan);

    // Append the section title (with arrow) to the section div
    sectionDiv.appendChild(sectionTitle);

    return sectionDiv;
  }

  createMenuItem(route) {
    const menuItemLink = document.createElement("a");
    menuItemLink.className = "menu-item";
    menuItemLink.href = route.url;
    menuItemLink.textContent = route.title;
    return menuItemLink;
  }

  toggleSubMenu(element) {
    const submenu = element.nextElementSibling;
    const isVisible = submenu.style.display === "block";
    submenu.style.display = isVisible ? "none" : "block";
    element.querySelector(".arrow").style.transform = isVisible
      ? ""
      : "rotate(360deg)";
  }

  changePageTitle(title) {
    document.title = title;
  }
}

const contentFetcher = new ContentFetcher();
const router = new Router(contentFetcher);
const init = async () => {
  await router.fetchRoutes();
  router.generateSidebarNavigation();
  router.navigate(window.location.pathname);
};

init();

const handleLinkClick = (event) => {
  event.preventDefault();
  event.stopPropagation();
  window.history.pushState({}, "", event.target.href);
  const url = new URL(event.target.href);
  router.navigate(url.pathname);
};

document.addEventListener("click", (event) => {
  if (
    event.target.matches("a.menu-item") ||
    (event.target.matches("a") &&
      event.target.href.startsWith(window.location.origin)) ||
    event.target.matches("a.menu-section-title")
  ) {
    handleLinkClick(event);
  }
  console.log(event.target);
});

window.addEventListener("popstate", () => {
  router.navigate(window.location.pathname);
});
