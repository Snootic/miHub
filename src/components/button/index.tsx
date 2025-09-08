import React from "react";
import './index.css'

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  text: string;
  variant: 'primary' | 'secondary' | 'danger'
}

export const Button: React.FC<ButtonProps> = (props) => {
  return (
    <button {...props} className={props.variant}>
      <p>{props.text}</p>
    </button>
  );
}