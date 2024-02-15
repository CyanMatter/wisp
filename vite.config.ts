import { defineConfig } from 'vite';
import { ViteRsw } from 'vite-plugin-rsw';

export default defineConfig({
  root: 'web',
  build: {
    minify: 'terser',
  },
  plugins: [
    ViteRsw(),
  ],
  base: "/web/dist/"
});
