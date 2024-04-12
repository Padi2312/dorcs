import type { Route } from "./types";

export class Router {
    private routes: Route[] = [];
    private currentRoute: Route | null = null;
    private onRouteChange: ((route: Route) => void) | null = null

    constructor() {
    }

    async init(onRouteChange: (route: Route) => void) {
        const response = await fetch("/routes.json");
        const data = await response.json();
        this.routes = data;
        this.onRouteChange = onRouteChange;
    }

    public navigate(url: string): void {
        // Convert url to URL object to get pathname
        // This is necessary to handle relative URLs
        const urlObj = new URL(url, window.location.href);
        url = urlObj.pathname;
        if (url == "/") {
            url = "/index.html";
        }

        const route = this.findRoute(url);
        if (route) {
            this.currentRoute = route;
            window.history.pushState({}, "", url);
            this.onRouteChange?.(route)
        }
    }

    public getCurrentRoute(): Route | null {
        return this.currentRoute;
    }

    public setCurrentRoute(url: string): void {
        this.currentRoute = this.findRoute(url);
    }

    public getRoutes(): Route[] {
        return this.routes;
    }

    public findRoute(url: string): Route | null {
        const findRoute = (routes: Route[]): Route | null => {
            for (const route of routes) {
                // Add `pages/` prefix to route.url to match the URL
                // This is necessary because the URL is relative to the root
                // of the server, but the route.url is relative to the `pages/` directory
                const new_url = `pages/${url.startsWith("/") ? url.slice(1) : url}`;
                if (route.url === new_url) {
                    return route;
                }

                if (route.children && route.children.length > 0) {
                    const foundRoute = findRoute(route.children);
                    if (foundRoute) {
                        return foundRoute;
                    }
                }
            }

            return null;
        };

        return findRoute(this.routes);
    }
}