'use client'

const Hero = () => {
  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden grid-background gradient-mesh bg-clay-100">
      {/* Animated background elements */}
      <div className="absolute inset-0 overflow-hidden">
        <div className="absolute top-20 left-10 w-72 h-72 bg-primary/8 rounded-full blur-3xl animate-float"></div>
        <div className="absolute bottom-20 right-10 w-96 h-96 bg-primary/5 rounded-full blur-3xl animate-float" style={{ animationDelay: '2s' }}></div>
      </div>

      <div className="container-custom relative z-10">
        <div className="max-w-5xl mx-auto text-center space-y-12 pt-32">
          {/* Main Heading */}
          <h1 className="text-display-lg lg:text-display-xxl font-bold text-warm-900 animate-fade-in-up">
            Deploy in 5 minutes.
          </h1>

          {/* Subheading */}
          <p className="text-2xl lg:text-3xl text-warm-600 max-w-2xl mx-auto leading-tight animate-fade-in-up animate-delay-100">
            Same contracts. Same tools. Zero changes.
          </p>

          {/* CTA Buttons */}
          <div className="flex flex-col sm:flex-row items-center justify-center gap-4 pt-8 animate-fade-in-up animate-delay-200">
            <a href="/docs" className="btn-primary text-lg px-10 py-5">
              Get Started
            </a>
            <a href="https://github.com/selendra" target="_blank" rel="noopener noreferrer" className="btn-outline text-lg px-10 py-5">
              View on GitHub
            </a>
          </div>
        </div>
      </div>

      {/* Scroll indicator */}
      <div className="absolute bottom-8 left-1/2 transform -translate-x-1/2 animate-bounce">
        <svg
          className="w-6 h-6 text-warm-500"
          fill="none"
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth="2"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path d="M19 14l-7 7m0 0l-7-7m7 7V3" />
        </svg>
      </div>
    </section>
  )
}

export default Hero
