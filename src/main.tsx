import { invoke } from "@tauri-apps/api/core";

import { StrictMode, useEffect } from "react";
import { AppContextProvider, useAppContext } from "./components/context/contextProvider";
import { createRoot } from "react-dom/client";
import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import CheckinForm from "./pages/checkinForm/index";
// import UpdaterPage from "./pages/updater/index";
import { Sidebar } from "./components";

import './styles.css'

const router = createBrowserRouter([
  {
    path: '/',
    element: <CheckinForm />,
  },
  // {
  //   path: '/updater',
  //   element: <UpdaterPage />,
  // },
])

function AppContent() {
  const { updateAvailable, setUpdateAvailable } = useAppContext();
  
  useEffect(() => {
    const checkUpdates = async () => {
      let updates
      try {
        updates = await invoke('check_for_updates')
        setUpdateAvailable(updates as boolean)
      } catch (e) {
        console.error(e)
      }
    }
    
    checkUpdates()
  })

  useEffect(() => {
    if (updateAvailable) {
      alert("ATUALIZAÇÃO ENCONTRADA")
      console.log(updateAvailable)
    }
  }, [updateAvailable])

  return (
    <main>
      <Sidebar />
      <RouterProvider router={router} />
    </main>
  );
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <AppContextProvider>
      <AppContent />
    </AppContextProvider>
  </StrictMode>
);
