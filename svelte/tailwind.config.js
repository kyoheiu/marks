/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors')

export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      spacing: {
        '120': '30rem',
        '144': '36rem',
      }
    },
  },
  plugins: [],
};
