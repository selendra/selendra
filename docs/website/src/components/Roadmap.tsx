'use client'

const Roadmap = () => {
  const phases = [
    {
      quarter: "2019-2025",
      status: "Completed",
      title: "Foundation Built",
      milestones: [
        "v1 mainnet launched (2022)",
        "v3 mainnet with AlephBFT + Aura (2025)",
        "30K users on StadiumX sports platform",
        "100+ merchants on Riverbase e-commerce",
        "Banking APIs integrated (ABA, ACLEDA, WING)"
      ]
    },
    {
      quarter: "Q4 2025",
      status: "In Progress",
      title: "Developer Experience",
      milestones: [
        "TypeScript SDK with full type safety",
        "Web-based IDE playground (5-min deploy)",
        "No-signup testnet faucet",
        "10+ production-ready templates",
        "Auto-generated docs from code"
      ]
    },
    {
      quarter: "Q1-Q2 2026",
      status: "Upcoming",
      title: "DeFi Infrastructure",
      milestones: [
        "Native DEX (10x cheaper than EVM)",
        "Chainlink oracle integration",
        "KHRt stablecoin (backed by banks)",
        "sUSD for DeFi (wrapped USDT/USDC)",
        "Staking with 5-10% APY"
      ]
    },
    {
      quarter: "Q3-Q4 2026",
      status: "Planned",
      title: "Cross-Chain & Scale",
      milestones: [
        "LayerZero bridge to Ethereum",
        "Advanced account abstraction",
        "Remove sudo, full governance",
        "Kun Khmer sports prediction (10 events/week)",
        "CPL fan tokens and NFT tickets"
      ]
    }
  ]

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Completed':
        return 'badge-success'
      case 'In Progress':
        return 'badge-primary'
      default:
        return 'badge-info'
    }
  }

  return (
    <section id="roadmap" className="section bg-clay-100">
      <div className="container-custom">
        <div className="text-center mb-16">
          <h2 className="text-display-lg font-bold text-warm-900 mb-6">
            Not a testnet.
            <br />
            <span className="gradient-text">A production chain.</span>
          </h2>
          <p className="text-xl text-warm-600 max-w-3xl mx-auto">
            Started 2019. Mainnet 2022. 30,000 users today. Now we help you build.
          </p>
        </div>

        <div className="relative">
          {/* Timeline line */}
          <div className="hidden lg:block absolute left-1/2 top-0 bottom-0 w-0.5 bg-gradient-to-b from-primary via-primary/50 to-transparent transform -translate-x-1/2"></div>

          <div className="space-y-12">
            {phases.map((phase, index) => (
              <div
                key={phase.quarter}
                className={`relative animate-fade-in ${
                  index % 2 === 0 ? 'lg:pr-1/2 lg:text-right' : 'lg:pl-1/2 lg:ml-auto'
                }`}
                style={{ animationDelay: `${index * 150}ms` }}
              >
                {/* Timeline dot */}
                <div className="hidden lg:block absolute top-8 left-1/2 w-4 h-4 bg-primary rounded-full transform -translate-x-1/2 ring-4 ring-clay-100 z-10 shadow-clay"></div>

                <div className="card-hover max-w-2xl">
                  <div className="flex items-center justify-between mb-4">
                    <div>
                      <div className="text-3xl font-bold text-warm-900 mb-2">{phase.quarter}</div>
                      <span className={`badge ${getStatusColor(phase.status)} text-xs`}>
                        {phase.status}
                      </span>
                    </div>
                  </div>

                  <h3 className="text-2xl font-semibold text-warm-900 mb-4">
                    {phase.title}
                  </h3>

                  <ul className="space-y-3">
                    {phase.milestones.map((milestone, idx) => (
                      <li key={idx} className="flex items-start text-warm-600">
                        <svg
                          className="w-5 h-5 text-primary mr-3 mt-0.5 flex-shrink-0"
                          fill="none"
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth="2"
                          viewBox="0 0 24 24"
                          stroke="currentColor"
                        >
                          <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <span>{milestone}</span>
                      </li>
                    ))}
                  </ul>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Call to action */}
        <div className="mt-16 text-center animate-fade-in animate-delay-500">
          <p className="text-warm-600 mb-6 text-lg">
            Building something? Let's talk.
          </p>
          <div className="flex flex-col sm:flex-row items-center justify-center gap-4">
            <a href="/docs" className="btn-primary">
              Start Building
            </a>
            <a href="https://github.com/selendra" target="_blank" rel="noopener noreferrer" className="btn-outline">
              View on GitHub
            </a>
          </div>
        </div>
      </div>
    </section>
  )
}

export default Roadmap
