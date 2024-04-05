export class ContentLoader {
    private cache: Map<string, Promise<string>>;
    constructor() {
        this.cache = new Map();
    }

    fetchContent(url: string) {
        if (!this.cache.has(url)) {
            const correctedUrl = new URL(url, window.location.href).toString();
            const contentPromise = fetch(correctedUrl).then((response) => response.text());
            this.cache.set(url, contentPromise);
        }
        return this.cache.get(url);
    }
}

