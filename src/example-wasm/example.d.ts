// import { WasiStderr as WasiStderrImports } from './imports/wasi-stderr';
// import { WasiIo as WasiIoImports } from './imports/wasi-io';
// import { WasiFilesystem as WasiFilesystemImports } from './imports/wasi-filesystem';
// import { WasiExit as WasiExitImports } from './imports/wasi-exit';
// import { WasiEnvironment as WasiEnvironmentImports } from './imports/wasi-environment';
import { Foo as FooExports } from './exports/foo';
import { $init } from "./exports"
export const foo: typeof FooExports;
export { $init };