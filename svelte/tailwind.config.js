/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors')

export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    colors: {
      background: "#e5e7eb", // gray-200
      text: "#111827", // gray-900
      item_background: "#f9fafb", // gray-50
      base_color: "#0ea5e9", // sky-500
      hovertitle: "#0284c7", // sky-600
      base_border: "#7dd3fc", // sky-300
      border: "#9ca3af", // gray-400
      desc: "#6b7280", // gray-500
      subtle: "#4b5563", // gray-600
      warning: "#b91c1c", // red-700
      unsaved: "#f87171", // red-400
      edit: "#14b8a6", // teal-500
      edit_border: "#5eead4", // teal-300
      further: "#d1d5db", // gray-300
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
