import { ItemsProvider } from '@app/context/ItemsContex';
import Layout from '@components/layout';
import About from '@pages/about';
import Error from '@pages/error';
import Galery from '@pages/galery';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';

const appRouter = createBrowserRouter([
  {
    element: <Layout />,
    errorElement: <Error />,
    path: '/',
    children: [
      {
        path: '/',
        element: (
          <ItemsProvider>
            <Galery />
          </ItemsProvider>
        ),
      },
      { path: '/about', element: <About /> },
    ],
  },
]);

const Router = () => {
  return <RouterProvider router={appRouter} />;
};

export default Router;
