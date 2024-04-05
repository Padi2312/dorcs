/** @type {import('tailwindcss').Config} */
export default {
  darkMode: "selector",
  content: ["./index.html", "./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors: {
        background: "var(--color-background)",
        textColor: "var(--color-text)",
        "table-body": "var(--color-table-body)",
        "table-header": "var(--color-table-header)",
        "code-background": "var(--color-code-background)",
      }
    }
  },
  plugins: [],
};
