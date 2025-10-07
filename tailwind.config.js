/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{js,ts,jsx,tsx,mdx}',],
  theme: {
    extend: {
      colors: {
        apple: {
          '50': '#f3faf3',
          '100': '#e2f6e4',
          '200': '#c7ebcb',
          '300': '#9bdaa2',
          '400': '#68c072',
          '500': '#47ae53',
          '600': '#32873c',
          '700': '#2b6a32',
          '800': '#26552c',
          '900': '#214627',
          '950': '#0d2611',
        },
      },
    }
  },
  extend: {},
  plugins: [],
}

