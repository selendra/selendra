import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, Routes, Route } from 'react-router-dom'
import App from './App.tsx'
import Docs from './pages/Docs.tsx'
import Navbar from './components/Navbar.tsx'
import Footer from './components/Footer.tsx'
import './styles/index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App />} />
        <Route
          path="/docs/:slug?"
          element={
            <>
              <Navbar />
              <Docs />
              <Footer />
            </>
          }
        />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>,
)
