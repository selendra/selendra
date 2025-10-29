import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'Selendra Network - EVM-Compatible Layer 1 Blockchain',
  description: 'The EVM-compatible Layer 1 blockchain with 1-second finality, Byzantine fault tolerance, and zero compromises. Built for Cambodia and Southeast Asia.',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="scroll-smooth">
      <head>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </head>
      <body className="antialiased transition-colors duration-300">{children}</body>
    </html>
  )
}
