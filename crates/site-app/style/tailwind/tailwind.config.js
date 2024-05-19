/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [ "./crates/**/*.rs" ],
  theme: {
    fontFamily: {
			'sans': [
        'inter', 'ui-sans-serif', 'system-ui', 'sans-serif', "Apple Color Emoji",
        "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"
      ],
    },
    screens: {
      'sm': '640px',
      'md': '768px',
      'lg': '1024px',
      'xl': '1280px',
      // '2xl': '1536px',
    },
    extend: {
      keyframes: {
        "slide-down": {
          '0%': { transform: 'translateY(-100%)', opacity: '0%' },
          '100%': { transform: 'translateY()', opacity: '100%' },
        }
      },
      animation: {
        "slide-down": 'slide-down 0.1s ease-out',
      },
    },
  },
  plugins: [
    require("rippleui"),
  ],
}
