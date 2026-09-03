import { defineConfig } from 'tsdown'

export default defineConfig({
  clean: true,
  entry: ['src/Api.ts', 'src/retry.ts', 'src/validate.ts'],
  format: ['cjs', 'esm'],
  dts: true,
  target: false,
})
