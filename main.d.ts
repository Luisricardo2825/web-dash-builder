/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface ConfigSchema {
  src?: string
  outDir?: string
  jsp?: Array<Jsp>
}
export interface Jsp {
  typeField: string
  code?: string
  path?: string
  varName?: string
}
export declare function build(arg?: ConfigSchema | string | undefined | null): boolean
