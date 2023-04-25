import { nodeResolve } from '@rollup/plugin-node-resolve'
import { terser } from 'rollup-plugin-terser'
import typescript from '@rollup/plugin-typescript'

export default {
  input: './js-guest/index.ts',
  output: {
    dir: './js-dist',
    entryFileNames: '[name].js',
    format: 'es',
    exports: 'auto'
  },
  plugins: [
    nodeResolve(),
    terser(),
    typescript({
      tsconfig: './tsconfig.json',
      moduleResolution: 'node'
    })
  ]
}
