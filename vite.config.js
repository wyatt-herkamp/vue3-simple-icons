
import { defineConfig } from 'vite';
import path from 'path';
import vue from '@vitejs/plugin-vue';


module.exports = defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@/': new URL('./src/', import.meta.url).pathname,
    },
  },
  build: {
    target: 'esnext',
    lib: {
      entry: path.resolve(__dirname, 'src/main.ts'),
      name: 'Vue3-simple-icons',
    },
    rollupOptions: {
      external: ['vue'],
      output: {
        globals: {
          vue: 'Vue'
        }
      }
    }
  }
})
