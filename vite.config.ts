import { defineConfig } from 'vite';
import { ViteRsw } from 'vite-plugin-rsw';

export default defineConfig({
  root: 'web',
  build: {
    minify: 'esbuild',
  },
  plugins: [
    ViteRsw(),
  ],
  base: "/wisp/"
});
