declare module "superuser" {
    type SuperUser = import("cockpit").EventMixin & {
        allowed: boolean | null;
        reload_page_on_change: () => void;
    }

    const superuser: SuperUser;
}
