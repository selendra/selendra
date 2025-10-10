import { useEffect, useState } from 'react'

export type Theme = 'light' | 'dark'

const getInitialTheme = (): Theme => {
  // Check if we're in the browser
  if (typeof window === 'undefined') return 'light'

  // Check localStorage first
  const stored = localStorage.getItem('theme') as Theme | null
  if (stored === 'light' || stored === 'dark') return stored

  // Check system preference
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return 'dark'
  }

  return 'light' // Default to light
}

export const useTheme = () => {
  const [theme, setTheme] = useState<Theme>(getInitialTheme)

  useEffect(() => {
    const root = document.documentElement

    if (theme === 'dark') {
      root.classList.add('dark')
      root.classList.remove('light')
    } else {
      root.classList.remove('dark')
      root.classList.add('light')
    }

    localStorage.setItem('theme', theme)
  }, [theme])

  const toggleTheme = () => {
    setTheme(prev => {
      const newTheme = prev === 'dark' ? 'light' : 'dark'
      console.log('Toggling theme from', prev, 'to', newTheme)
      return newTheme
    })
  }

  return { theme, toggleTheme }
}
