'use client'

import { useEffect } from 'react'
import Hero from '@/components/Hero'
import Features from '@/components/Features'
import Technology from '@/components/Technology'
import Stats from '@/components/Stats'
import Roadmap from '@/components/Roadmap'
import CTA from '@/components/CTA'
import Footer from '@/components/Footer'
import Navbar from '@/components/Navbar'

export default function Home() {
  useEffect(() => {
    // Smooth scroll behavior
    document.documentElement.style.scrollBehavior = 'smooth'
  }, [])

  return (
    <div className="min-h-screen bg-clay-100">
      <Navbar />
      <Hero />
      <Stats />
      <Features />
      <Technology />
      <Roadmap />
      <CTA />
      <Footer />
    </div>
  )
}
