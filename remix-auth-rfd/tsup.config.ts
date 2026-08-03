import { defineConfig } from 'tsup'

export default defineConfig({
  clean: true,
  entry: ['src/index.ts'],
  format: ['cjs', 'esm'],
  // tsup unconditionally injects `baseUrl: '.'` into the DTS build, which TS 6 flags as deprecated.
  // https://github.com/egoist/tsup/issues/1388
  dts: {
    compilerOptions: {
      ignoreDeprecations: '6.0',
    },
  },
})
