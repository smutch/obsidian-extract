/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [ "template.html" ],
  theme: {
    extend: {},
    container: {
      center: true,
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('@tailwindcss/forms'),
  ],
}
