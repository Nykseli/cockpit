type SpawnConfig = import("cockpit").SpawnConfig;
type SpawnPromise = import("cockpit").SpawnPromise;

declare module "python" {
    function spawn(script_pieces: string | string[], args: string[], options: SpawnConfig): SpawnPromise
}
