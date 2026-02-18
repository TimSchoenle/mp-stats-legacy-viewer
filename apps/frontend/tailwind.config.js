/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.rs",
        "./index.html",
    ],
    theme: {
        extend: {
            colors: {
                minecraft: {
                    dark: '#1D1D1D',
                    dirt: '#866043',
                    grass: '#5E8135',
                    emerald: '#17DD62',
                },
                dark: {
                    950: '#0a0a0a',
                    900: '#171717',
                    850: '#202020',
                    800: '#262626',
                }
            },
            fontFamily: {
                sans: ['Inter', 'sans-serif'],
                mono: ['Fira Code', 'monospace'],
            },
        },
    },
    plugins: [],
}
