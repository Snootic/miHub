import React from "react";
import './index.css'
import { Button } from "../button";

import { invoke } from "@tauri-apps/api/core";

interface sidebarProps {

}

export const Sidebar: React.FC<sidebarProps> = (props) => {
  return (
    <div className="sidebar" {...props}>
      <Button variant="secondary" text={"Update"} onClick={() => invoke('start_update')}/>
    </div>
  )
}