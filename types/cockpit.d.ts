// https://github.com/cockpit-project/cockpit/pull/13214

declare module "cockpit" {
	interface Func1<T, R = void> {
		(arg: T): R;
	}

	export type DbusOptions = {
		bus?: "session" | "user" | "system" | "internal" | "none";
		address?: string;
		superuser?: "require" | "try";
		track?: boolean;
	};

	export type Fail = {
		message: string;
		problem?: string;
	};

	export type SpawnFail = Fail & {
		exit_status?: number;
		exit_signal?: number;
	};

	export type ErrorConfig = "message" | "out" | "ignore" | "pty";

	/**
	 * https://github.com/cockpit-project/cockpit/blob/main/src/bridge/cockpitrouter.c#L615-L621
	 * @deprecated boolean is tecnically valid but it's not well documented
	 */
	export type SuperUserBool = boolean;
	export type Superuser = "require" | "try" | SuperUserBool;
	export type ProblemCodes =
		| "access-denied"
		| "authentication-failed"
		| "internal-error"
		| "no-cockpit"
		| "no-session"
		| "not-found"
		| "terminated"
		| "timeout"
		| "unknown-hostkey"
		| "no-forwarding";

	export type SpawnConfig = {
		err?: ErrorConfig;
		binary?: boolean;
		directory?: string;
		host?: string;
		environ?: string[];
		pty?: boolean;
		batch?: boolean;
		latency?: number;
		message?: string;
		superuser?: Superuser;
	};

	export type ProxyMethods<T extends Record<string, (...args: any[]) => any>> = {
		[k in keyof T]: T[k]
	}

	export type Proxy<T extends Record<string, (...args: any[]) => any> = {}> = ProxyMethods<T> & {
		client: DbusClient;
		path: string;
		iface: string;
		valid: boolean;
		data: Object;
		wait: (callback: () => void) => Promise<void>;
	}

	export type DbusEvent = "close" | "owner";

	export type DBusEventCallback<T extends DbusEvent> =
		T extends "close" ? (event: CustomEvent<unknown>, options: { problem?: string }) => void :
		T extends "owner" ? (event: CustomEvent<unknown>, owner?: string | null) => void :
		never;

	// Special type for "cockpit.Superuser" proxy interface
	export type SuperUserProxy = Proxy & {
		// TODO: more complete types from documentation
		Current: "none" | "pseudo" | "root" | "init";
		// TODO: event_mixin
		addEventListener(event: string, callback: () => void): void;
	}

	export type SpecialProxyName = "cockpit.Superuser";

	export type CheckSpecialProxy<N extends SpecialProxyName | string, T> =
		N extends "cockpit.Superuser" ? SuperUserProxy : T

	export interface DbusClient {
		wait: (callback: () => void) => Promise<void>;
		close(problem?: string): void;
		// TODO: Add "cockpit.SuperUser" interface type for js stuff, see superuser.js
		proxy<T extends Record<string, (...args: any[]) => any> = {}, N extends SpecialProxyName | string>(interface?: N, path?: string): EventMixin & CheckSpecialProxy<N, Proxy<T>>;
		proxies(interface?: string[], path?: string[]): Proxy[];
		subscribe(arg1: any, arg2: any): any;
		addEventListener<T extends DbusEvent>(event: T, callback: DBusEventCallback<T>): void;
		options: DbusOptions;
		unique_name: string;
	}

	export interface ClosableWithProblem {
		close(problem?: ProblemCodes): void;
	}

	export interface SpawnPromise extends Promise<string>, ClosableWithProblem {
		stream(callback: Func1<string>): SpawnPromise;
		input(data?: string | Uint8Array, stream?: boolean): SpawnPromise;
		then(callback: (arg: unknown) => void): void; // todo
	}

	export type PermissionArgs = {
		admin?: boolean;
		group?: string;
		user?: {
			id: number;
			name: string;
			groups: string[] | null;
		};
		_is_superuser?: boolean;
	}

	export type Permission = {
		admin: boolean;
		user: PermissionArgs["user"] | null;
		is_superuser: boolean | null;
		group: string;
		allowed: boolean | null;
		// TODO: event_mixin
		addEventListener(event: string, callback: () => void): void;
	}

	export type EventMixin = {
		addEventListener(event: string, callback: () => void): void;
		removeEventListener(event: string, callback: () => void): void;
		dispatchEvent(event: string): void;
	}

	export type MessageArg = {
		/** If message is defined, it will will be returned */
		message?: string;
		/** If problem is set, it returns a custom tranlated message */
		problem?:
		"terminated" |
		"no-session" |
		"access-denied" |
		"authentication-failed" |
		"authentication-not-supported" |
		"unknown-hostkey" |
		"unknown-host" |
		"invalid-hostkey" |
		"internal-error" |
		"timeout" |
		"no-cockpit" |
		"no-forwarding" |
		"disconnected" |
		"not-supported" |
		"no-host" |
		"too-large"
	}

	export type Transport = EventMixin & {
		application: string;
		ready: boolean;

		close(options: {problem?: string, command?: string}): void
		next_channel(): void;
		// TODO; types
		send_data(data: any, channel: any, control: any): bool;
		// TODO; types
		send_message(payload: any, channel: any, control: any): bool;
		// TODO; types
		send_control(data: any): bool;
		// TODO; types
		register(channel: any, control_cb: any, message_cb: any): void;
		// TODO; types
		unregister(channel: void): void;
	}

	export interface Cockpit {
		gettext(text: string): string;
		gettext(context: string, text: string): string;
		format(template: string, args: string | Object): string;
		format(template: string, ...args: string[] | Object[]): string;
		dbus(name: string | null, options?: DbusOptions): DbusClient;
		jump(todo: string, host?: string | null): void;
		script(script: string, args: SpawnConfig): SpawnPromise;
		spawn(args: string | string[], options?: SpawnConfig): SpawnPromise;
		event_target<T = object>(obj: T): EventMixin & T;
		permission(args?: PermissionArgs): Permission;
		empty_event_mixin(): EventMixin;
		message(arg: MessageArg): string

		transport: { host?: string | null };
		language: string;
	}

	declare const cockpit: Cockpit;
	export default cockpit;
}
