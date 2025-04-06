import type { ConfigSchema } from "./main";
import type { PluginOption } from "vite";
export interface Options {
  devConfig?: ConfigSchema;
  prodConfig?: ConfigSchema;
}

export declare function plugin(options?: Options): PluginOption[];

export function build(options?: ConfigSchema, entryFile?: string): boolean;
export declare interface Window {
  __IP_SERVER__: string;
}

export * from "./main";
