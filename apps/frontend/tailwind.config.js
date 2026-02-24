/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.rs",
        "./index.html",
    ],
    theme: {
        extend: {
            fontFamily: {
                sans: ['Inter', 'sans-serif'],
                mono: ['Fira Code', 'monospace'],
            },
        },
    },
    plugins: [],
}
