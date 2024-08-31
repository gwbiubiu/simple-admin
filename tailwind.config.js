/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["web/src/**/*.rs", "index.html"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui") ],
  daisyui: {
    themes: ["light"],
  },
}

