'use client'

import { useState, useEffect } from 'react'
import Link from 'next/link'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import rehypeHighlight from 'rehype-highlight'
import rehypeRaw from 'rehype-raw'
import 'highlight.js/styles/atom-one-light.css'
import Navbar from '@/components/Navbar'
import Footer from '@/components/Footer'

interface Doc {
  slug: string
  title: string
  section: string
  order: number
  content: string
}

interface Section {
  name: string
  docs: Doc[]
}

interface DocsPageProps {
  params: {
    slug?: string[]
  }
}

export default function DocsPage({ params }: DocsPageProps) {
  const slug = params.slug?.[0] || undefined
  const [sections, setSections] = useState<Section[]>([])
  const [currentDoc, setCurrentDoc] = useState<Doc | null>(null)
  const [loading, setLoading] = useState(true)
  const [expandedSection, setExpandedSection] = useState<string | null>(null)

  useEffect(() => {
    // Load all available docs
    const loadDocs = async () => {
      const docFiles = [
        'getting-started',
        'network-overview',
        'run-a-node',
        'become-validator',
        'developers',
        'smart-contracts'
      ]

      const loadedDocs: Doc[] = []

      for (const file of docFiles) {
        try {
          const response = await fetch(`/docs/${file}.md`)
          const text = await response.text()

          // Parse frontmatter
          const frontmatterRegex = /^---\n([\s\S]*?)\n---\n([\s\S]*)$/
          const match = text.match(frontmatterRegex)

          if (match) {
            const frontmatter = match[1]
            const content = match[2]

            const titleMatch = frontmatter.match(/title:\s*(.+)/)
            const orderMatch = frontmatter.match(/order:\s*(\d+)/)
            const sectionMatch = frontmatter.match(/section:\s*(.+)/)

            loadedDocs.push({
              slug: file,
              title: titleMatch ? titleMatch[1] : file,
              section: sectionMatch ? sectionMatch[1] : 'Other',
              order: orderMatch ? parseInt(orderMatch[1]) : 999,
              content
            })
          }
        } catch (error) {
          console.error(`Failed to load ${file}:`, error)
        }
      }

      loadedDocs.sort((a, b) => a.order - b.order)

      // Group docs by section
      const sectionMap = new Map<string, Doc[]>()
      loadedDocs.forEach(doc => {
        if (!sectionMap.has(doc.section)) {
          sectionMap.set(doc.section, [])
        }
        sectionMap.get(doc.section)!.push(doc)
      })

      const sectionsArray: Section[] = Array.from(sectionMap.entries()).map(([name, docs]) => ({
        name,
        docs
      }))

      setSections(sectionsArray)

      // Set current doc
      const current = slug
        ? loadedDocs.find(d => d.slug === slug)
        : loadedDocs[0]

      setCurrentDoc(current || null)

      // Open the section containing the current doc by default
      if (current && sectionsArray.length > 0) {
        setExpandedSection(current.section)
      } else if (sectionsArray.length > 0) {
        setExpandedSection(sectionsArray[0].name)
      }

      setLoading(false)
    }

    loadDocs()
  }, [slug])

  if (loading) {
    return (
      <>
        <Navbar />
        <div className="min-h-screen bg-clay-100 pt-20 flex items-center justify-center">
          <div className="text-warm-600">Loading documentation...</div>
        </div>
        <Footer />
      </>
    )
  }

  return (
    <>
      <Navbar />
      <div className="min-h-screen bg-clay-100 pt-20">
        <div className="container-custom">
          <div className="flex gap-8 py-12">
            {/* Sidebar */}
            <aside className="w-64 flex-shrink-0">
              <div className="sticky top-24 clay-card p-6">
                <h2 className="text-lg font-bold text-warm-900 mb-4">Documentation</h2>
                <nav className="space-y-4">
                  {sections.map((section) => (
                    <div key={section.name}>
                      <Link
                        href={`/docs/${section.docs[0].slug}`}
                        onClick={() => {
                          setExpandedSection(section.name)
                        }}
                        className="flex items-center justify-between w-full text-sm font-semibold text-warm-700 hover:text-warm-900 transition-colors mb-2"
                      >
                        {section.name}
                        <svg
                          className={`w-4 h-4 transition-transform ${
                            expandedSection === section.name ? 'rotate-90' : ''
                          }`}
                          fill="none"
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth="2"
                          viewBox="0 0 24 24"
                          stroke="currentColor"
                        >
                          <path d="M9 5l7 7-7 7" />
                        </svg>
                      </Link>
                      {expandedSection === section.name && (
                        <div className="space-y-1 ml-2 border-l border-clay-300/50 pl-3">
                          {section.docs.map((doc) => (
                            <Link
                              key={doc.slug}
                              href={`/docs/${doc.slug}`}
                              className={`block px-3 py-2 rounded-clay text-sm transition-colors ${
                                currentDoc?.slug === doc.slug
                                  ? 'bg-primary/10 text-primary font-medium shadow-clay-sm'
                                  : 'text-warm-600 hover:text-warm-900 hover:bg-clay-100'
                              }`}
                            >
                              {doc.title}
                            </Link>
                          ))}
                        </div>
                      )}
                    </div>
                  ))}
                </nav>
              </div>
            </aside>

            {/* Main content */}
            <main className="flex-1 max-w-4xl">
              {currentDoc ? (
                <article className="clay-card p-8 prose prose-slate max-w-none">
                  <ReactMarkdown
                    remarkPlugins={[remarkGfm]}
                    rehypePlugins={[rehypeHighlight, rehypeRaw]}
                    components={{
                      h1: ({ node, ...props }) => (
                        <h1 className="text-4xl font-bold text-warm-900 mb-6" {...props} />
                      ),
                      h2: ({ node, ...props }) => (
                        <h2 className="text-3xl font-bold text-warm-900 mt-12 mb-4" {...props} />
                      ),
                      h3: ({ node, ...props }) => (
                        <h3 className="text-2xl font-bold text-warm-900 mt-8 mb-3" {...props} />
                      ),
                      p: ({ node, ...props }) => (
                        <p className="text-warm-600 leading-relaxed mb-4" {...props} />
                      ),
                      a: ({ node, ...props }) => (
                        <a className="text-primary hover:text-primary-600 underline" {...props} />
                      ),
                      code: ({ node, inline, ...props }: any) =>
                        inline ? (
                          <code className="bg-clay-200 text-primary px-1.5 py-0.5 rounded text-sm" {...props} />
                        ) : (
                          <code className="block bg-clay-100 p-4 rounded-clay overflow-x-auto border border-clay-300/30" {...props} />
                        ),
                      ul: ({ node, ...props }) => (
                        <ul className="list-disc list-inside text-warm-600 space-y-2 mb-4" {...props} />
                      ),
                      ol: ({ node, ...props }) => (
                        <ol className="list-decimal list-inside text-warm-600 space-y-2 mb-4" {...props} />
                      ),
                      pre: ({ node, ...props }) => (
                        <pre className="bg-clay-100 p-4 rounded-clay overflow-x-auto mb-6 border border-clay-300/30" {...props} />
                      ),
                    }}
                  >
                    {currentDoc.content}
                  </ReactMarkdown>
                </article>
              ) : (
                <div className="text-center py-12">
                  <p className="text-warm-600">Documentation not found</p>
                </div>
              )}
            </main>
          </div>
        </div>
      </div>
      <Footer />
    </>
  )
}
