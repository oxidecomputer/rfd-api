import { defineConfig } from 'tsdown'

export default defineConfig({
  clean: true,
  entry: ['src/Api.ts', 'src/retry.ts', 'src/validate.ts', 'src/msw-handlers.ts'],
  format: ['esm'],
  dts: true,
  // tsdown defaults ESM output to .mjs/.d.mts; the package is `type: module`,
  // so keep the plain .js/.d.ts names the published `exports` point at.
  outExtensions: () => ({ js: '.js', dts: '.d.ts' }),
})
