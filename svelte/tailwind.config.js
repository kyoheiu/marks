/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors')

export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    colors: {
      background: "#e5e7eb", // gray-200
      text: "#111827", // gray-900
      item_background: "#f9fafb", // gray-50
      base: "#0284c7", // sky-600
      border: "#9ca3af", // gray-400
      subtle: "#4b5563", // gray-600
      warning: "#b91c1c", // red-700
      edit: "#0d9488", // teal-600
    },
    extend: {
      spacing: {
        '120': '30rem',
        '144': '36rem',
      }
    },
  },
  plugins: [],
};
