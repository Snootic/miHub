import React, { createContext, useContext, useState, ReactNode } from 'react';

interface AppContextProps {
  updateAvailable: boolean
  setUpdateAvailable: (updateAvailable: boolean) => void
}

const AppContext = createContext<AppContextProps | undefined>(undefined);

export const useAppContext = () => {
  const context = useContext(AppContext);
  if (!context) {
    throw new Error('useAppContext must be used within an AppContextProvider');
  }
  return context;
};

interface AppContextProviderProps {
  children: ReactNode;
}

export const AppContextProvider: React.FC<AppContextProviderProps> = ({ children }) => {
  const [updateAvailable, setUpdateAvailable] = useState<boolean>(false);

  return (
    <AppContext.Provider value={{ updateAvailable, setUpdateAvailable }}>
      {children}
    </AppContext.Provider>
  );
};