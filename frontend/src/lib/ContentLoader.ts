export class ContentLoader {
    private cache: Map<string, Promise<string>>;
    constructor() {
        this.cache = new Map();
    }

    fetchContent(url: string) {
        if (!this.cache.has(url)) {
            // Get domain wihtout paths
            const domain = window.location.origin;
            const preparedUrl = `${url}.html`
            const correctedUrl = new URL(preparedUrl, domain).toString();
            const contentPromise = fetch(correctedUrl).then((response) => response.text());
            this.cache.set(url, contentPromise);
        }
        return this.cache.get(url);
    }
}

