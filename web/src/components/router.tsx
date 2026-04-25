import { createBrowserRouter } from 'react-router'
import { Home } from './features/home/Home'

export const router = createBrowserRouter([
	{
		path: '/',
		element: <Home />,
	},
]);

