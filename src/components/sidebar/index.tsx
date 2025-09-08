import React from "react";
import './index.css'
import { Button } from "../button";

interface sidebarProps {

}

export const Sidebar: React.FC<sidebarProps> = (...props) => {
  return (
    <div className="sidebar">
      <Button variant="secondary" text={"Update"}/>
    </div>
  )
}