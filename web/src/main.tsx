import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { RouterProvider } from 'react-router'
import { router } from './components/router'
import { Header } from './components/organisms/header/Header'
import './index.css'

function App() {
  return (
    <>
			<Header />
			<RouterProvider router={router} />
    </>
  )
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
