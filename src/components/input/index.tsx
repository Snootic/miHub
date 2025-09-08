import React from "react"
import './index.css'
type InputProps = React.InputHTMLAttributes<HTMLInputElement>

export const Input: React.FC<InputProps> = (props) => {
  return (
    <input
      className="input"
      {...props}
    />
  )
}