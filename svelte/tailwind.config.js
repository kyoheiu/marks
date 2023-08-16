/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors')

export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    colors: {
      background: "#e5e7eb", // gray-200
      text: "#111827", // gray-900
      itembackground: "#f9fafb", // gray-50
      lightbuttontext: "#f9fafb", // gray-50
      subtle: "#4b5563", // gray-600
      desc: "#6b7280", // gray-500
      further: "#9ca3af", // gray-400
      basecolor: "#2563eb", // blue-600
      baseborder: "#60a5fa", // blue-400
      hovertitle: "#2563eb", // blue-600
      warning: "#b91c1c", // red-700
      saved: "#1d4ed8", // blue-700
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
