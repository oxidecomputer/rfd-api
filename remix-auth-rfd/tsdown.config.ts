import { defineConfig } from 'tsdown'

export default defineConfig({
  clean: true,
  entry: ['src/index.ts'],
  format: ['esm'],
  dts: true,
  // Keep .js/.d.ts filenames for this type: module package.
  outExtensions: () => ({ js: '.js', dts: '.d.ts' }),
})
