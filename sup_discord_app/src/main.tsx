import { StrictMode} from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import "./styles/_globals.scss";

createRoot(document.getElementById("root")! as HTMLElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
