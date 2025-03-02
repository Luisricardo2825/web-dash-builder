import type { ConfigSchema } from "./main";
import type { Plugin } from "vite";
export interface PluginOptions {
  devConfig?: ConfigSchema;
  prodConfig?: ConfigSchema;
};

export declare function plugin(options?: PluginOptions): Plugin[];

export function build(options?: ConfigSchema, entryFile?: string): boolean;
export declare interface Window {
  __IP_SERVER__: string;
}

export * from "./main";
