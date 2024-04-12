export class ContentLoader {
    private cache: Map<string, Promise<string>>;
    constructor() {
        this.cache = new Map();
    }

    fetchContent(url: string, force = false) {
        if (!this.cache.has(url) || force) {
            // Get domain wihtout paths
            const domain = window.location.origin;
            const preparedUrl = `${url}.html`
            const correctedUrl = new URL(preparedUrl, domain).toString();
            console.log(correctedUrl)
            const contentPromise = fetch(correctedUrl).then((response) => response.text());
            this.cache.set(url, contentPromise);
        }
        return this.cache.get(url);
    }
}

