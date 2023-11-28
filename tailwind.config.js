export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  theme: {
    extend: {
      colors: {
          foreground: 'rgba(var(--foreground), <alpha-value>)',
          background: 'rgb(var(--background), <alpha-value>)',
          muted: {
            DEFAULT: 'rgba(var(--muted), <alpha-value>)',
            gray: 'rgba(var(--muted-gray), <alpha-value>)',
            foregorund: 'rgba(var(--muted-foreground), <alpha-value>)',
          },
          gray: 'rgba(var(--gray), <alpha-value>)',
          border: 'rgba(var(--border), <alpha-value>)',
        }
    },
  },
  plugins: [],
}

