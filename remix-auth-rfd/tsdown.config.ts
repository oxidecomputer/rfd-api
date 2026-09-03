import { defineConfig } from 'tsdown'

export default defineConfig({
  clean: true,
  entry: ['src/index.ts'],
  format: ['cjs', 'esm'],
  dts: true,
  target: false,
})
