import { defineConfig } from 'tsup'

export default defineConfig({
  clean: true,
  entry: ['src/Api.ts', 'src/retry.ts', 'src/validate.ts'],
  format: ['cjs', 'esm'],
  splitting: false,
  // tsup unconditionally injects `baseUrl: '.'` into the DTS build, which TS 6 flags as deprecated.
  // https://github.com/egoist/tsup/issues/1388
  dts: {
    compilerOptions: {
      ignoreDeprecations: '6.0',
    },
  },
})
