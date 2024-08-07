import type { ConfigSchema } from "./main";
import type { Plugin } from "vite";
export function plugin(options?: {
  devConfig?: ConfigSchema;
  prodConfig?: ConfigSchema;
}): Plugin[];

export function build(options?: ConfigSchema): boolean;
export declare interface Window {
  __IP_SERVER__: string;
}

export * from "./main";
