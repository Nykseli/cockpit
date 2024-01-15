declare global {
	interface CockpitWindow {
		debugging?: string;
		mock?: {
			url: string;
			pathname: string;
			url_root: string;
			// TODO: define the private Transport class type
			last_transport: any
		};
		options: {
			protocol: string;
		};
		cockpit: import("cockpit").Cockpit
	}

	interface Window extends CockpitWindow { }
}

// This file needs to be treated as a module for the global types to register
export { }
