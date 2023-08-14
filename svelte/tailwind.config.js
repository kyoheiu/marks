/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors')

export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    colors: {
      background: "#e5e7eb", // gray-200
      text: "#111827", // gray-900
      item_background: "#f9fafb", // gray-50
      base_color: "#0284c7", // sky-600
      base_border: "#7dd3fc", // sky-300
      border: "#9ca3af", // gray-400
      subtle: "#4b5563", // gray-600
      warning: "#b91c1c", // red-700
      unsaved: "#f87171", // red-400
      saved: "#059669", // emerald-600
      edit: "#0d9488", // teal-600
      edit_border: "#5eead4", // teal-300
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
