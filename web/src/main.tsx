import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { RouterProvider } from 'react-router'
import { router } from './components/router'
import { Header } from './components/organisms/header/Header'
import './index.css'
import { Sidenav } from './components/organisms/sidenav/Sidenav'

function App() {
  return (
    <>
			<div className="flex w-full">
				<Sidenav />
				<div className="flex flex-col w-full">
					<Header />
					<RouterProvider router={router} />
				</div>
			</div>
		</>
  )
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
