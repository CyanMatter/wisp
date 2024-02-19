import { defineConfig } from 'vite';
import { ViteRsw } from 'vite-plugin-rsw';

export default defineConfig({
  server: {
    open: true,
    port: 3000,
  },
  root: 'web',
  build: {
    minify: 'esbuild',
  },
  plugins: [
    ViteRsw(),
  ],
  base: "/wisp/",
});
