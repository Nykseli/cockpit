type SpawnPromise = import("cockpit").SpawnPromise;
type EventMixin = import("cockpit").EventMixin;

declare module "service" {
	type ServiceProxy = EventMixin & {
		exists: boolean | null;
		state: "starting" | "running" | "stopping" | "stopped" |"failed"| null | undefined;
		enabled: boolean | null | undefined;
		// TODO: Proxy type
		unit?: any;
		// TODO: Proxy type
		details?: any;
		// TODO: Proxy type
		service?: any;

		wait: (callback: () => void) => void;
		start: () => Promise<void>;
		stop: () => Promise<void>;
		restart: () => Promise<void>;
		tryRestart: () => Promise<void>;
		enable: () => any;
		disable: () => any;
		getRunJournal: (options: any) => SpawnPromise | Promise<never>
	}

	function proxy(name: string, kind?: string): ServiceProxy;
}
