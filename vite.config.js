import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue';
import dynamicImportVariables from "@rollup/plugin-dynamic-import-vars";
const { resolve } = require('path')

module.exports = defineConfig({
  plugins: [vue()],
  build: {
    target: 'esnext',
    lib: {
      entry: resolve(__dirname, 'src/main.js'),
      name: 'Vue3-simple-icons',
    },
    rollupOptions: {
      plugins: [dynamicImportVariables()],
      // make sure to externalize deps that shouldn't be bundled
      // into your library
      external: ['vue'],
      output: {
        // Provide global variables to use in the UMD build
        // for externalized deps
        globals: {
          vue: 'Vue'
        }
      }
    }
  }
})
