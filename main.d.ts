/* auto-generated by NAPI-RS */
/* eslint-disable */
export declare function build(arg?: ConfigSchema | string | undefined | null): boolean

export interface ConfigSchema {
  src?: string
  outDir?: string
  jsp?: Array<Jsp>
  defaultHeaders?: boolean
  baseFolder?: string
}

export interface Jsp {
  typeField: string
  code?: string
  path?: string
  varName?: string
}
