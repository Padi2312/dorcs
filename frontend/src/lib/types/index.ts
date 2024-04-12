export type Route = {
    children: null | Route[];
    position: number;
    title: string;
    url: string;
};

export type PageSettings = {
    page_title: string
    icon: string
    dev: boolean
}