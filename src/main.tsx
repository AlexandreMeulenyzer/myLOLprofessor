import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router-dom";

import { router } from "@/app/router";
import { AppProviders } from "@/app/providers";
import "@/styles/index.css";

const rootElement = document.getElementById("root");
if (!rootElement) {
  throw new Error("Root element #root introuvable dans index.html");
}

ReactDOM.createRoot(rootElement).render(
  <React.StrictMode>
    <AppProviders>
      <RouterProvider router={router} />
    </AppProviders>
  </React.StrictMode>,
);
