const colors = require('tailwindcss/colors')
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,ts}"],
  theme: {
    extend: {
      colors: {
        transparent: 'transparent',
        current: 'currentColor',
        black: colors.black,
        white: colors.white,
        emerald: colors.emerald,
        indigo: colors.indigo,
        yellow: colors.yellow,
        lime: colors.lime,
        rose: colors.rose,
      }
    }
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}

