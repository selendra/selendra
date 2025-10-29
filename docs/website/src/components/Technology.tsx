'use client'

const Technology = () => {
  const techStack = [
    {
      name: "Hybrid Consensus",
      description: "Aura makes blocks. AlephBFT finalizes them. Fast and secure.",
      features: ["1000ms blocks", "<2s finality", "BFT security"]
    },
    {
      name: "Full EVM",
      description: "Your Solidity code. MetaMask. Hardhat. Remix. All of it. Just works.",
      features: ["Frontier runtime", "15M+ gas/block", "EVM precompiles"]
    },
    {
      name: "Unified Accounts",
      description: "One account. EVM and native. No bridges.",
      features: ["Single balance", "Cross-runtime calls", "Seamless UX"]
    },
    {
      name: "Account Abstraction",
      description: "Lost your keys? Your friends recover them. Session keys for dApps. No more 12-word panic.",
      features: ["Guardian recovery", "Temporary keys", "Gasless txs"]
    }
  ]

  return (
    <section id="technology" className="section bg-clay-50">
      <div className="container-custom">
        <div className="text-center mb-16">
          <h2 className="text-display-lg font-bold text-warm-900 mb-6">
            Technology that
            <br />
            <span className="gradient-text">gets out of your way.</span>
          </h2>
          <p className="text-xl text-warm-600 max-w-3xl mx-auto">
            Built on Cardinal Cryptography's Polkadot SDK. Production-tested, not experimental.
          </p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-16">
          {techStack.map((tech, index) => (
            <div
              key={tech.name}
              className="card-hover animate-fade-in"
              style={{ animationDelay: `${index * 100}ms` }}
            >
              <h3 className="text-3xl font-bold text-warm-900 mb-4">{tech.name}</h3>
              <p className="text-warm-600 mb-6 leading-relaxed text-lg">
                {tech.description}
              </p>
              <div className="flex flex-wrap gap-2">
                {tech.features.map((feature) => (
                  <span
                    key={feature}
                    className="px-3 py-1 bg-clay-200 border border-clay-300/50 rounded-full text-xs text-warm-700 shadow-clay-sm"
                  >
                    {feature}
                  </span>
                ))}
              </div>
            </div>
          ))}
        </div>

        {/* Runtime Stats */}
        <div className="card animate-fade-in animate-delay-400">
          <h3 className="text-2xl font-bold text-warm-900 mb-8 text-center">Runtime v3.0 (Spec 20004)</h3>
          <div className="grid grid-cols-2 lg:grid-cols-4 gap-8">
            <div className="text-center">
              <div className="text-4xl font-bold gradient-text mb-2">30</div>
              <div className="text-warm-600 text-sm">Total Pallets</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold gradient-text mb-2">7</div>
              <div className="text-warm-600 text-sm">EVM Precompiles</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold gradient-text mb-2">1961</div>
              <div className="text-warm-600 text-sm">EVM Chain ID</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold gradient-text mb-2">2019</div>
              <div className="text-warm-600 text-sm">Project Started</div>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}

export default Technology
