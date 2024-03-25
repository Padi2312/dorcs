export type Route = {
    children: null | Route[];
    is_section: boolean;
    position: number;
    title: string;
    url: string;
};