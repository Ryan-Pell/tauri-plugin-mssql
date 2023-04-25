import { nodeResolve } from '@rollup/plugin-node-resolve'
import { terser } from 'rollup-plugin-terser'
import typescript from '@rollup/plugin-typescript'

export default {
  input: './js-guest/index.ts',
  output: {
    dir: './js-dist',
    entryFileNames: '[name].js',
    format: 'es',
    exports: 'auto',
    sourcemap: true,
    plugins: []
  },
  plugins: [
    nodeResolve(),
    terser(),
    typescript({
      tsconfig: './js-guest/tsconfig.json',
      moduleResolution: 'node',
    })
  ]
}
