/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },
  theme: {
    extend: {
      colors: {
        'dark': {
          '50': '#f8fafc',
          '100': '#f1f5f9',
          '200': '#e2e8f0',
          '300': '#cbd5e1',
          '400': '#94a3b8',
          '500': '#64748b',
          '600': '#475569',
          '700': '#334155',
          '800': '#1e293b',
          '900': '#0f172a',
          '950': '#020617',
        },
        'slate': {
          '50': '#f8fafc',
          '100': '#f1f5f9',
          '200': '#e2e8f0',
          '300': '#cbd5e1',
          '400': '#94a3b8',
          '500': '#64748b',
          '600': '#475569',
          '700': '#334155',
          '800': '#1e293b',
          '900': '#0f172a',
          '950': '#020617',
        }
      }
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        "wwe-gm-dark": {
          "primary": "#6366f1",        // Indigo for primary actions
          "primary-content": "#ffffff",
          "secondary": "#8b5cf6",      // Purple for roster management
          "secondary-content": "#ffffff",
          "accent": "#06b6d4",         // Cyan for championships
          "accent-content": "#ffffff",
          "neutral": "#1e293b",        // Deep slate for neutral elements
          "neutral-content": "#f1f5f9",
          "base-100": "#020617",       // Darker base for WWE GM feel
          "base-200": "#0f172a",       // Slate-950 equivalent
          "base-300": "#1e293b",       // Slate-800 equivalent
          "base-content": "#f1f5f9",   // Light text on dark background
          "info": "#0ea5e9",
          "info-content": "#ffffff",
          "success": "#10b981",
          "success-content": "#ffffff",
          "warning": "#f59e0b",
          "warning-content": "#ffffff",
          "error": "#ef4444",
          "error-content": "#ffffff",
        },
      },
      "dark",
      "light",
    ],
  },
};
