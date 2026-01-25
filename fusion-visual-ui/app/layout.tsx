import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'Fusion Visual Compiler',
  description: 'Quantum-Native Systems Programming Interface',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
