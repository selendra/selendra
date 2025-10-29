export default function DocsLoading() {
  return (
    <div className="min-h-screen bg-clay-100 flex items-center justify-center">
      <div className="text-center">
        <div className="inline-flex items-center justify-center w-16 h-16 mb-4">
          <div className="w-16 h-16 border-4 border-clay-300 border-t-primary rounded-full animate-spin"></div>
        </div>
        <p className="text-warm-600 text-lg">Loading documentation...</p>
      </div>
    </div>
  )
}
