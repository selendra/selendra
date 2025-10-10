import { useEffect } from 'react'
import Hero from './components/Hero'
import Features from './components/Features'
import Technology from './components/Technology'
import Stats from './components/Stats'
import Roadmap from './components/Roadmap'
import CTA from './components/CTA'
import Footer from './components/Footer'
import Navbar from './components/Navbar'

function App() {
  useEffect(() => {
    // Smooth scroll behavior
    document.documentElement.style.scrollBehavior = 'smooth'
  }, [])

  return (
    <div className="min-h-screen bg-dark-900">
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

export default App
