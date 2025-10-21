const Technology = () => {
  const techStack = [
    {
      name: "Substrate",
      category: "Framework",
      description: "Framework from Parity Technologies. Powers Polkadot. Modular pallets and forkless upgrades.",
      features: ["FRAME pallets", "Runtime upgrades", "Cross-chain ready"]
    },
    {
      name: "AlephBFT",
      category: "Finality",
      description: "BFT consensus from Cardinal Cryptography. Sub-2s finality. Used by Aleph Zero in production.",
      features: ["BFT security", "Fast finality", "High throughput"]
    },
    {
      name: "Aura",
      category: "Block Production",
      description: "Round-robin block production. 1s blocks with low variance. Standard in Substrate chains.",
      features: ["1s blocks", "Deterministic", "Low variance"]
    },
    {
      name: "Frontier",
      category: "EVM Integration",
      description: "Ethereum compatibility for Substrate. Run EVM contracts, support Web3 RPC. Your Ethereum tools work.",
      features: ["EVM runtime", "Web3 RPC", "Ethereum tooling"]
    }
  ]

  return (
    <section id="technology" className="section bg-white dark:bg-dark-900">
      <div className="container-custom">
        <div className="text-center mb-16">
          <span className="badge-info text-sm mb-4 inline-block">Tech Stack</span>
          <h2 className="text-display-lg font-bold text-slate-900 dark:text-white mb-4">
            What we're
            <span className="gradient-text"> built on</span>
          </h2>
          <p className="text-xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto">
            Proven components from the Substrate/Polkadot ecosystem. Production-tested, not experimental.
          </p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-16">
          {techStack.map((tech, index) => (
            <div
              key={tech.name}
              className="card-hover animate-fade-in"
              style={{ animationDelay: `${index * 100}ms` }}
            >
              <div className="flex items-start justify-between mb-4">
                <div>
                  <h3 className="text-3xl font-bold text-slate-900 dark:text-white mb-2">{tech.name}</h3>
                  <span className="badge-primary text-xs">{tech.category}</span>
                </div>
              </div>
              <p className="text-slate-600 dark:text-slate-400 mb-6 leading-relaxed">
                {tech.description}
              </p>
              <div className="flex flex-wrap gap-2">
                {tech.features.map((feature) => (
                  <span
                    key={feature}
                    className="px-3 py-1 bg-slate-200 dark:bg-slate-800/50 border border-slate-300 dark:border-slate-700 rounded-full text-xs text-slate-700 dark:text-slate-300"
                  >
                    {feature}
                  </span>
                ))}
              </div>
            </div>
          ))}
        </div>

        {/* Architecture Diagram */}
        <div className="card animate-fade-in animate-delay-400">
          <h3 className="text-2xl font-bold text-slate-900 dark:text-white mb-8 text-center">Hybrid Consensus Architecture</h3>
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
            <div className="text-center">
              <div className="w-16 h-16 mx-auto mb-4 rounded-2xl bg-primary/10 border border-primary/20 flex items-center justify-center">
                <svg className="w-8 h-8 text-primary" fill="none" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" viewBox="0 0 24 24" stroke="currentColor">
                  <path d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
                </svg>
              </div>
              <h4 className="text-xl font-semibold text-slate-900 dark:text-white mb-2">Block Production</h4>
              <p className="text-slate-600 dark:text-slate-400 text-sm">
                Aura validators produce blocks every second in round-robin fashion
              </p>
            </div>

            <div className="text-center">
              <div className="w-16 h-16 mx-auto mb-4 rounded-2xl bg-primary/10 border border-primary/20 flex items-center justify-center">
                <svg className="w-8 h-8 text-primary" fill="none" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" viewBox="0 0 24 24" stroke="currentColor">
                  <path d="M13 10V3L4 14h7v7l9-11h-7z" />
                </svg>
              </div>
              <h4 className="text-xl font-semibold text-slate-900 dark:text-white mb-2">Fast Finality</h4>
              <p className="text-slate-600 dark:text-slate-400 text-sm">
                AlephBFT consensus finalizes blocks in under 2 seconds with BFT guarantees
              </p>
            </div>

            <div className="text-center">
              <div className="w-16 h-16 mx-auto mb-4 rounded-2xl bg-primary/10 border border-primary/20 flex items-center justify-center">
                <svg className="w-8 h-8 text-primary" fill="none" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" viewBox="0 0 24 24" stroke="currentColor">
                  <path d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                </svg>
              </div>
              <h4 className="text-xl font-semibold text-slate-900 dark:text-white mb-2">EVM Execution</h4>
              <p className="text-slate-600 dark:text-slate-400 text-sm">
                Frontier runtime executes Ethereum transactions with full compatibility
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}

export default Technology
