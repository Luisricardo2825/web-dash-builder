import type { ConfigSchema } from "./main";
import type { Plugin } from "vite";
export function plugin(
  devBuildConfig?: ConfigSchema,
  prodBuildConfig?: ConfigSchema
): Plugin[];

export declare interface Window {
  __IP_SERVER__: string;
}

export * from "./main";
