export type Route = {
    children: null | Route[];
    is_section: boolean;
    position: number;
    title: string;
    url: string;
};

export type PageSettings = {
    page_title: string
    icon: string
    landing_page: string
}