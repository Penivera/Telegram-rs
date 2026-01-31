const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class', // Enable class-based dark mode
  theme: {
    extend: {
      colors: {
        background: {
          light: '#f8f9fc',
          dark: '#0f1117',
        },
        surface: {
          light: '#ffffff',
          dark: '#1f2937',
        },
        text: {
          light: '#1f2937',
          dark: '#f9fafb',
        },
        secondaryText: {
          light: '#6b7280',
          dark: '#9ca3af',
        },
        primary: {
          light: '#229ED9',
          DEFAULT: '#0088cc',
          dark: '#006699',
        },
      },
      fontFamily: {
        sans: ['Inter', ...defaultTheme.fontFamily.sans],
        mono: ['JetBrains Mono', ...defaultTheme.fontFamily.mono],
      },
      maxWidth: {
        content: '75ch',
      },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
};