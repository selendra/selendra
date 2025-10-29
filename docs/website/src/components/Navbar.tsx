'use client'

import { useState, useEffect } from 'react'
import Link from 'next/link'

const Navbar = () => {
  const [isScrolled, setIsScrolled] = useState(false)
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false)

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 20)
    }

    window.addEventListener('scroll', handleScroll)
    return () => window.removeEventListener('scroll', handleScroll)
  }, [])

  const navLinks = [
    { name: 'Features', href: '/#features' },
    { name: 'Technology', href: '/#technology' },
    { name: 'Roadmap', href: '/#roadmap' },
    { name: 'Docs', href: '/docs' },
  ]

  return (
    <nav
      className={`fixed top-0 left-0 right-0 z-50 transition-all duration-300 ${
        isScrolled
          ? 'bg-white/80 backdrop-blur-lg border-b border-clay-300/30 shadow-clay'
          : 'bg-transparent'
      }`}
    >
      <div className="container-custom">
        <div className="flex items-center justify-between h-16 lg:h-20">
          {/* Logo */}
          <Link href="/" className="flex items-center space-x-2">
            <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-primary to-primary-700 flex items-center justify-center shadow-clay-sm">
              <span className="text-white font-bold text-xl">S</span>
            </div>
            <span className="text-xl font-bold text-warm-900">Selendra</span>
          </Link>

          {/* Desktop Navigation */}
          <div className="hidden lg:flex items-center space-x-8">
            {navLinks.map((link) => (
              link.name === 'Docs' ? (
                <Link
                  key={link.name}
                  href={link.href}
                  className="text-warm-600 hover:text-warm-900 transition-colors duration-200 font-medium"
                >
                  {link.name}
                </Link>
              ) : (
                <a
                  key={link.name}
                  href={link.href}
                  className="text-warm-600 hover:text-warm-900 transition-colors duration-200 font-medium"
                >
                  {link.name}
                </a>
              )
            ))}
          </div>

          {/* CTA Buttons */}
          <div className="hidden lg:flex items-center space-x-4">
            <a
              href="#"
              className="text-warm-600 hover:text-warm-900 transition-colors duration-200 font-medium"
            >
              GitHub
            </a>
            <a href="#" className="btn-primary">
              Launch App
            </a>
          </div>

          {/* Mobile menu button */}
          <div className="lg:hidden flex items-center space-x-2">
            <button
              onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
              className="p-2 rounded-lg hover:bg-clay-200 transition-colors"
            >
              <svg
                className="w-6 h-6 text-warm-600"
                fill="none"
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                {isMobileMenuOpen ? (
                  <path d="M6 18L18 6M6 6l12 12" />
                ) : (
                  <path d="M4 6h16M4 12h16M4 18h16" />
                )}
              </svg>
            </button>
          </div>
        </div>

        {/* Mobile menu */}
        {isMobileMenuOpen && (
          <div className="lg:hidden py-4 border-t border-clay-300/30 animate-fade-in-down">
            <div className="flex flex-col space-y-4">
              {navLinks.map((link) => (
                link.name === 'Docs' ? (
                  <Link
                    key={link.name}
                    href={link.href}
                    className="text-warm-600 hover:text-warm-900 transition-colors duration-200 py-2 font-medium"
                    onClick={() => setIsMobileMenuOpen(false)}
                  >
                    {link.name}
                  </Link>
                ) : (
                  <a
                    key={link.name}
                    href={link.href}
                    className="text-warm-600 hover:text-warm-900 transition-colors duration-200 py-2 font-medium"
                    onClick={() => setIsMobileMenuOpen(false)}
                  >
                    {link.name}
                  </a>
                )
              ))}
              <div className="pt-4 border-t border-clay-300/30 flex flex-col space-y-3">
                <a href="#" className="btn-secondary w-full">
                  GitHub
                </a>
                <a href="#" className="btn-primary w-full">
                  Launch App
                </a>
              </div>
            </div>
          </div>
        )}
      </div>
    </nav>
  )
}

export default Navbar
