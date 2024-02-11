const { scopedPreflightStyles } = require('tailwindcss-scoped-preflight');

module.exports = {
    darkMode: 'class',
    content: ["./src/**/*.rs"],
    plugins: [
        scopedPreflightStyles({
            cssSelector: '.leptos-kbar',
            mode: 'matched only', // it's the default
        })
    ],
    theme: {
        extend: {
            colors: {
                'smoke': 'hsla(0, 0%, 0%, 0.05)',
            }
        },
    },
}