const Roadmap = () => {
  const phases = [
    {
      quarter: "Q4 2024",
      status: "Completed",
      title: "Foundation & Testnet",
      milestones: [
        "Core runtime development with Substrate",
        "AlephBFT consensus integration",
        "EVM compatibility via Frontier",
        "Testnet launch with 4+ validators",
        "Basic governance implementation"
      ]
    },
    {
      quarter: "Q1 2025",
      status: "In Progress",
      title: "Mainnet Preparation",
      milestones: [
        "Security audits (runtime + consensus)",
        "Performance optimization and benchmarking",
        "Validator onboarding program",
        "Developer documentation and SDK",
        "Community testnet incentives"
      ]
    },
    {
      quarter: "Q2 2025",
      status: "Upcoming",
      title: "Mainnet Launch",
      milestones: [
        "Mainnet genesis with initial validators",
        "Token generation event (TGE)",
        "CEX listings and liquidity provision",
        "Block explorer and network monitoring",
        "Developer grants program launch"
      ]
    },
    {
      quarter: "Q3-Q4 2025",
      status: "Planned",
      title: "Ecosystem Growth",
      milestones: [
        "DeFi protocol deployments (DEX, lending)",
        "Bridge to Ethereum and other L1s",
        "NFT marketplace and gaming integrations",
        "Enterprise partnership program",
        "Advanced governance features"
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
    <section id="roadmap" className="section bg-white dark:bg-slate-950/50">
      <div className="container-custom">
        <div className="text-center mb-16">
          <span className="badge-primary text-sm mb-4 inline-block">Roadmap</span>
          <h2 className="text-display-lg font-bold text-slate-900 dark:text-white mb-4">
            From Testnet to
            <span className="gradient-text"> Production Scale</span>
          </h2>
          <p className="text-xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto">
            A clear path to mainnet launch and ecosystem growth, with transparency
            at every milestone.
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
                <div className="hidden lg:block absolute top-8 left-1/2 w-4 h-4 bg-primary rounded-full transform -translate-x-1/2 ring-4 ring-dark-900 z-10"></div>

                <div className="card-hover max-w-2xl">
                  <div className="flex items-center justify-between mb-4">
                    <div>
                      <div className="text-3xl font-bold text-slate-900 dark:text-white mb-2">{phase.quarter}</div>
                      <span className={`badge ${getStatusColor(phase.status)} text-xs`}>
                        {phase.status}
                      </span>
                    </div>
                  </div>

                  <h3 className="text-2xl font-semibold text-slate-900 dark:text-white mb-4">
                    {phase.title}
                  </h3>

                  <ul className="space-y-3">
                    {phase.milestones.map((milestone, idx) => (
                      <li key={idx} className="flex items-start text-slate-600 dark:text-slate-400">
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
          <p className="text-slate-600 dark:text-slate-400 mb-6">
            Want to stay updated on our progress?
          </p>
          <div className="flex flex-col sm:flex-row items-center justify-center gap-4">
            <a href="#" className="btn-primary">
              Join Community
            </a>
            <a href="#" className="btn-outline">
              Follow Development
            </a>
          </div>
        </div>
      </div>
    </section>
  )
}

export default Roadmap
