const Stats = () => {
  const stats = [
    {
      value: "1s",
      label: "Block Time",
      description: "Consistent block production with Aura consensus"
    },
    {
      value: "<2s",
      label: "Finality",
      description: "AlephBFT provides fast cryptographic finality"
    },
    {
      value: "$0.001",
      label: "Average Fee",
      description: "Low enough for micro-transactions and frequent transfers"
    },
    {
      value: "100%",
      label: "EVM Compatible",
      description: "Full Ethereum tooling support via Frontier"
    }
  ]

  return (
    <section className="section bg-white dark:bg-dark-900">
      <div className="container-custom">
        <div className="text-center mb-16">
          <h2 className="text-display-lg font-bold text-slate-900 dark:text-white mb-4">
            Mainnet
            <span className="gradient-text"> performance</span>
          </h2>
          <p className="text-xl text-slate-600 dark:text-slate-400 max-w-2xl mx-auto">
            Current metrics
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
          {stats.map((stat, index) => (
            <div
              key={stat.label}
              className="card-hover text-center group animate-fade-in"
              style={{ animationDelay: `${index * 100}ms` }}
            >
              <div className="text-6xl lg:text-7xl font-bold gradient-text mb-3 group-hover:scale-110 transition-transform duration-300">
                {stat.value}
              </div>
              <div className="text-2xl font-semibold text-slate-900 dark:text-white mb-4">
                {stat.label}
              </div>
              <div className="text-slate-600 dark:text-slate-400 leading-relaxed">
                {stat.description}
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}

export default Stats
