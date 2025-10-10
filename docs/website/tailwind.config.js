/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Anthropic-inspired palette with Selendra's primary color
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
        dark: {
          DEFAULT: '#0f1419',
          50: '#f7f8f9',
          100: '#eceef1',
          200: '#d4d9de',
          300: '#b0b8c1',
          400: '#8691a0',
          500: '#687182',
          600: '#535b6b',
          700: '#444a57',
          800: '#3a3f4a',
          900: '#0f1419', // Main dark
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
    },
  },
  plugins: [],
}
