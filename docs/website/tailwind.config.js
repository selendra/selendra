/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        // Clay morphism palette - warm, earthy, and soft
        primary: {
          DEFAULT: '#c6613f', // Selendra primary (coral/terracotta)
          50: '#fef6f3',
          100: '#fde9e1',
          200: '#fad2c2',
          300: '#f6b398',
          400: '#f08a62',
          500: '#c6613f', // Main
          600: '#b54d2a',
          700: '#9a3f21',
          800: '#7f341d',
          900: '#6a2e1c',
        },
        // Clay background colors - warm, soft, organic
        clay: {
          DEFAULT: '#f5f1ed', // Main clay background
          50: '#fdfcfb',
          100: '#faf8f5',
          200: '#f5f1ed', // Main
          300: '#ebe4dc',
          400: '#dfd4c8',
          500: '#cfc1b0',
          600: '#b5a593',
          700: '#968777',
          800: '#766a5f',
          900: '#5c5249',
        },
        // Warm neutrals for text and borders
        warm: {
          DEFAULT: '#6b5d54',
          50: '#faf8f6',
          100: '#f0ebe6',
          200: '#e3d9d0',
          300: '#cebfb2',
          400: '#b5a391',
          500: '#988272',
          600: '#6b5d54', // Main text
          700: '#574b43',
          800: '#473d37',
          900: '#3a322d',
        },
        slate: {
          DEFAULT: '#64748b',
          50: '#f8fafc',
          100: '#f1f5f9',
          200: '#e2e8f0',
          300: '#cbd5e1',
          400: '#94a3b8',
          500: '#64748b',
          600: '#475569',
          700: '#334155',
          800: '#1e293b',
          900: '#0f172a',
        },
      },
      fontFamily: {
        sans: [
          'Inter',
          '-apple-system',
          'BlinkMacSystemFont',
          'Segoe UI',
          'Roboto',
          'sans-serif',
        ],
        mono: [
          'JetBrains Mono',
          'Fira Code',
          'Monaco',
          'Consolas',
          'monospace',
        ],
      },
      fontSize: {
        'display-xxl': ['5rem', { lineHeight: '1', letterSpacing: '-0.02em' }],
        'display-xl': ['4rem', { lineHeight: '1.1', letterSpacing: '-0.02em' }],
        'display-lg': ['3.5rem', { lineHeight: '1.1', letterSpacing: '-0.02em' }],
        'display-md': ['3rem', { lineHeight: '1.2', letterSpacing: '-0.01em' }],
        'display-sm': ['2.5rem', { lineHeight: '1.2', letterSpacing: '-0.01em' }],
        'heading-xl': ['2rem', { lineHeight: '1.3', letterSpacing: '-0.01em' }],
        'heading-lg': ['1.75rem', { lineHeight: '1.3', letterSpacing: '-0.005em' }],
        'heading-md': ['1.5rem', { lineHeight: '1.4', letterSpacing: '0' }],
        'heading-sm': ['1.25rem', { lineHeight: '1.4', letterSpacing: '0' }],
      },
      animation: {
        'fade-in': 'fadeIn 0.6s ease-out forwards',
        'fade-in-up': 'fadeInUp 0.6s ease-out forwards',
        'fade-in-down': 'fadeInDown 0.6s ease-out forwards',
        'slide-in-left': 'slideInLeft 0.6s ease-out forwards',
        'slide-in-right': 'slideInRight 0.6s ease-out forwards',
        'scale-in': 'scaleIn 0.4s ease-out forwards',
        'glow': 'glow 2s ease-in-out infinite alternate',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        fadeInUp: {
          '0%': { opacity: '0', transform: 'translateY(20px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        fadeInDown: {
          '0%': { opacity: '0', transform: 'translateY(-20px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        slideInLeft: {
          '0%': { opacity: '0', transform: 'translateX(-40px)' },
          '100%': { opacity: '1', transform: 'translateX(0)' },
        },
        slideInRight: {
          '0%': { opacity: '0', transform: 'translateX(40px)' },
          '100%': { opacity: '1', transform: 'translateX(0)' },
        },
        scaleIn: {
          '0%': { opacity: '0', transform: 'scale(0.95)' },
          '100%': { opacity: '1', transform: 'scale(1)' },
        },
        glow: {
          '0%': { boxShadow: '0 0 20px rgba(198, 97, 63, 0.3)' },
          '100%': { boxShadow: '0 0 40px rgba(198, 97, 63, 0.6)' },
        },
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-conic': 'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
      },
      boxShadow: {
        // Clay morphism shadows - soft, layered, organic
        'clay-sm': '0 2px 4px rgba(107, 93, 84, 0.06), 0 1px 2px rgba(107, 93, 84, 0.04)',
        'clay': '0 4px 8px rgba(107, 93, 84, 0.08), 0 2px 4px rgba(107, 93, 84, 0.06)',
        'clay-md': '0 8px 16px rgba(107, 93, 84, 0.1), 0 4px 8px rgba(107, 93, 84, 0.08)',
        'clay-lg': '0 16px 32px rgba(107, 93, 84, 0.12), 0 8px 16px rgba(107, 93, 84, 0.1)',
        'clay-xl': '0 24px 48px rgba(107, 93, 84, 0.14), 0 12px 24px rgba(107, 93, 84, 0.12)',
        'clay-inner': 'inset 0 2px 4px rgba(107, 93, 84, 0.08)',
        'clay-glow': '0 0 20px rgba(198, 97, 63, 0.15), 0 4px 8px rgba(107, 93, 84, 0.08)',
      },
      borderRadius: {
        'clay': '16px',
        'clay-lg': '24px',
        'clay-xl': '32px',
      },
    },
  },
  plugins: [],
}
