const Hero = () => {
  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden grid-background gradient-mesh bg-slate-50 dark:bg-dark-900">
      {/* Animated background elements */}
      <div className="absolute inset-0 overflow-hidden">
        <div className="absolute top-20 left-10 w-72 h-72 bg-primary/10 dark:bg-primary/10 rounded-full blur-3xl animate-float"></div>
        <div className="absolute bottom-20 right-10 w-96 h-96 bg-primary/5 dark:bg-primary/5 rounded-full blur-3xl animate-float" style={{ animationDelay: '2s' }}></div>
      </div>

      <div className="container-custom relative z-10">
        <div className="max-w-5xl mx-auto text-center space-y-8 pt-20">
          {/* Badge */}
          <div className="animate-fade-in-down">
            <span className="badge-primary text-sm">
              <span className="inline-block w-2 h-2 rounded-full bg-primary mr-2 animate-pulse"></span>
              Mainnet Live
            </span>
          </div>

          {/* Main Heading - BIG TYPOGRAPHY */}
          <h1 className="text-display-lg lg:text-display-xxl font-bold text-slate-900 dark:text-white animate-fade-in-up animate-delay-100">
            The Easiest Blockchain
            <br />
            <span className="gradient-text">To Build On. Period.</span>
          </h1>

          {/* Subheading */}
          <p className="text-xl lg:text-2xl text-slate-600 dark:text-slate-300 max-w-3xl mx-auto leading-relaxed animate-fade-in-up animate-delay-200">
            Full Ethereum compatibility. Deploy in minutes.
            Built for Cambodia and Southeast Asia—cross-border payments, supply chain, financial access.
          </p>

          {/* CTA Buttons */}
          <div className="flex flex-col sm:flex-row items-center justify-center gap-4 pt-4 animate-fade-in-up animate-delay-300">
            <a href="#" className="btn-primary text-lg px-8 py-4 group">
              Start Building
              <svg
                className="w-5 h-5 ml-2 group-hover:translate-x-1 transition-transform"
                fill="none"
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path d="M13 7l5 5m0 0l-5 5m5-5H6" />
              </svg>
            </a>
            <a href="#" className="btn-outline text-lg px-8 py-4">
              Read Docs
            </a>
          </div>

          {/* Key Metrics */}
          <div className="grid grid-cols-2 lg:grid-cols-4 gap-8 pt-16 animate-fade-in animate-delay-400">
            <div className="space-y-2">
              <div className="text-4xl lg:text-5xl font-bold gradient-text">1s</div>
              <div className="text-slate-600 dark:text-slate-400 text-sm lg:text-base">Block Time</div>
            </div>
            <div className="space-y-2">
              <div className="text-4xl lg:text-5xl font-bold gradient-text">&lt;2s</div>
              <div className="text-slate-600 dark:text-slate-400 text-sm lg:text-base">Finality</div>
            </div>
            <div className="space-y-2">
              <div className="text-4xl lg:text-5xl font-bold gradient-text">$0.001</div>
              <div className="text-slate-600 dark:text-slate-400 text-sm lg:text-base">Avg Fee</div>
            </div>
            <div className="space-y-2">
              <div className="text-4xl lg:text-5xl font-bold gradient-text">100%</div>
              <div className="text-slate-600 dark:text-slate-400 text-sm lg:text-base">EVM Compatible</div>
            </div>
          </div>

          {/* Tech Stack Logos */}
          <div className="pt-16 animate-fade-in animate-delay-500">
            <p className="text-slate-500 dark:text-slate-500 text-sm mb-6">Built with</p>
            <div className="flex flex-wrap items-center justify-center gap-8 opacity-60">
              <div className="text-slate-600 dark:text-slate-400 font-mono text-sm">Substrate</div>
              <div className="text-slate-600 dark:text-slate-400 font-mono text-sm">AlephBFT</div>
              <div className="text-slate-600 dark:text-slate-400 font-mono text-sm">Frontier</div>
              <div className="text-slate-600 dark:text-slate-400 font-mono text-sm">Aura</div>
            </div>
          </div>
        </div>
      </div>

      {/* Scroll indicator */}
      <div className="absolute bottom-8 left-1/2 transform -translate-x-1/2 animate-bounce">
        <svg
          className="w-6 h-6 text-slate-500 dark:text-slate-500"
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
