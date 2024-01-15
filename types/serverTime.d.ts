declare module "serverTime" {
    type NtpStatus = {
        initialized: boolean;
        active: boolean;
        synch: boolean;
        service: string | null;
        server: string | null;
        sub_status: string | null;
    };
}
