import { invoke } from "@tauri-apps/api/core";

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import CheckinForm from "./pages/checkinForm/index";
import { Sidebar } from "./components";

import './styles.css'

const router = createBrowserRouter([
  {
    path: '/',
    element: <CheckinForm />,
  },
])

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <main>
      <Sidebar />
      <RouterProvider router={router} />
    </main>
  </StrictMode>
);
